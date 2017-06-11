use std::collections::HashSet;

use mem_map::*;
use bootrom::Bootrom;
use cartridge::Cartridge;
use memory::Memory;
use gpu::Gpu;
use device::Device;
use apu::Apu;
use timer::Timer;
use gamepad::Gamepad;
use interrupt::Irq;
use events::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaState {
    Inactive,
    Setup1,
    Setup2,
    Reset1,
    Reset2,
    Active(u16),
}

pub struct Interconnect {
    boot_rom: Bootrom,
    cartridge: Cartridge,
    gpu: Gpu,
    apu: Apu,
    timer: Timer,
    gamepad: Gamepad,

    boot_rom_active: bool,

    internal_ram: Memory,
    high_ram: Memory,
    pub if_register: u8,
    pub ie_register: u8,

    serial_transfer_data: u8,
    serial_control: u8,

    trigger_watchpoint: bool,
    pub watchpoints: HashSet<u16>,

    pub dma_source: u16,
    pub dma_slot: u8,
    pub dma_state: DmaState,
}

impl Interconnect {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge) -> Interconnect {
        Interconnect {
            boot_rom: bootrom,
            cartridge: cartridge,
            gpu: Gpu::new(),
            apu: Apu::new(),
            timer: Timer::default(),
            gamepad: Gamepad::new(),

            boot_rom_active: true,

            internal_ram: Memory::new(INTERNAL_RAM_LENGTH),
            high_ram: Memory::new(HIGH_RAM_END),

            if_register: 0,
            ie_register: 0,

            serial_transfer_data: 0,
            serial_control: 0,

            watchpoints: HashSet::new(),
            trigger_watchpoint: false,

            dma_source: 0,
            dma_slot: 0,
            dma_state: DmaState::Inactive,
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms, match_overlapping_arm))]
    pub fn read_byte(&self, addr: u16) -> u8 {
        if !(self.dma_state == DmaState::Inactive || self.dma_state == DmaState::Setup1 || self.dma_state == DmaState::Setup2) {
            let ext_bus_1 = |x| x < 0x8000;
            let vram_bus = |x| (x >= 0x8000) && (x < 0xa000);
            let ext_bus_2 = |x| (x >= 0xa000) && (x < 0xfe00);

            let dma_source = self.dma_source + match self.dma_state {
                DmaState::Setup1 => 0,
                DmaState::Setup2 => 0,
                DmaState::Reset1 => 0,
                DmaState::Reset2 => 0,
                DmaState::Active(index) => index + 1,
                _ => unreachable!(),
            };

            if (ext_bus_1(addr) && ext_bus_1(dma_source)) ||
               (vram_bus(addr) && vram_bus(dma_source)) ||
               (ext_bus_2(addr) && ext_bus_2(dma_source)) {
                   return self.dma_slot;
               }
        }

        self.inner_read_byte(addr)
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms, match_overlapping_arm))]
    pub fn write_byte(&mut self, addr: u16, val: u8) {
        if self.watchpoints.contains(&addr) {
            self.trigger_watchpoint = true;
        }

        match addr {
            ROM_START...ROM_END => self.cartridge.write(addr - ROM_START, val),
            VRAM_START...VRAM_END => self.gpu.write_vram(addr - VRAM_START, val),
            CRAM_START...CRAM_END => self.cartridge.write(addr - ROM_START, val),
            INTERNAL_RAM_START...INTERNAL_RAM_END => {
                self.internal_ram.write_byte(addr - INTERNAL_RAM_START, val)
            }
            IRAM_ECHO_START...IRAM_ECHO_END => {
                self.internal_ram.write_byte(addr - IRAM_ECHO_START, val)
            }
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.write_byte(addr - HIGH_RAM_START, val),
            OAM_START...OAM_END => if self.dma_state != DmaState::Inactive {} else { self.gpu.write_oam(addr - OAM_START, val)},
            0xff00 => self.gamepad.write_reg(val),

            // TODO - implement serial port (Link cable)
            0xff01 => self.serial_transfer_data = val,
            0xff02 => self.serial_control = val,

            0xff04...0xff07 => self.timer.write_reg(addr, val),
            0xff0f => self.if_register = val,
            0xff10...0xff3f => self.apu.write_reg(addr, val),
            0xff46 => {
                self.dma_source = (val as u16) << 8;
                if self.dma_state != DmaState::Inactive {
                    self.dma_state = DmaState::Reset1;
                } else {
                    self.dma_state = DmaState::Setup1;
                }
            }
            0xff40...0xff4b => self.gpu.write_reg(addr, val),
            0xff50 => self.boot_rom_active = false,
            0xffff => self.ie_register = val,
            _ => {} // Writes to unused addresses have no effect
        }
    }

    pub fn step(&mut self, cycles: u16, device: &mut Device, events: &mut Vec<Event>) {
        for _ in 0..(cycles / 4) {
            match self.dma_state {
                DmaState::Inactive => {}
                DmaState::Setup1 => {
                    self.dma_state = DmaState::Setup2;
                }
                DmaState::Reset1 => {
                    self.dma_state = DmaState::Reset2;
                }
                DmaState::Setup2 | DmaState::Reset2 => {
                    self.dma_slot = self.inner_read_byte(self.dma_source);
                    self.dma_state = DmaState::Active(0);
                }
                DmaState::Active(index) => {
                    let val = self.dma_slot;
                    self.dma_slot = self.inner_read_byte(self.dma_source + index + 1);
                    self.gpu.write_oam(index, val);

                    if index >= 159 {
                        self.dma_state = DmaState::Inactive;
                    } else {
                        self.dma_state = DmaState::Active(index + 1);
                    }
                }
            }
        }

        let mut irq = Irq::default();

        self.apu.step(cycles, device, &mut irq);
        self.gpu.step(cycles, device, &mut irq);
        self.timer.step(cycles, device, &mut irq);
        self.gamepad.step(cycles, device, &mut irq);

        self.if_register |= irq.get_if();

        if self.trigger_watchpoint {
            events.push(Event::Watchpoint);
            self.trigger_watchpoint = false;
        }
    }

    pub fn get_width(&self) -> usize {
        self.gpu.get_width()
    }

    pub fn get_height(&self) -> usize {
        self.gpu.get_height()
    }

    pub fn get_timer(&self) -> &Timer {
        &self.timer
    }

    fn inner_read_byte(&self, addr: u16) -> u8 {
        match addr {
            BOOT_ROM_START...BOOT_ROM_END if self.boot_rom_active => self.boot_rom.read_byte(addr - BOOT_ROM_START),
            ROM_START...ROM_END => self.cartridge.read_byte(addr - ROM_START),
            VRAM_START...VRAM_END => self.gpu.read_vram(addr - VRAM_START),
            CRAM_START...CRAM_END => self.cartridge.read_byte(addr - ROM_START),
            INTERNAL_RAM_START...INTERNAL_RAM_END => {
                self.internal_ram.read_byte(addr - INTERNAL_RAM_START)
            }
            IRAM_ECHO_START...IRAM_ECHO_END => self.internal_ram.read_byte(addr - IRAM_ECHO_START),
            OAM_START...OAM_END => {
                if !(self.dma_state == DmaState::Inactive || self.dma_state == DmaState::Setup1 || self.dma_state == DmaState::Setup2) {
                    0xff
                } else {
                    self.gpu.read_oam(addr - OAM_START)
                }
            }
            0xff00 => self.gamepad.read_reg(),

            // TODO - implement serial port (Link cable)
            0xff01 => 0x00,
            0xff02 => 0x7e,

            0xff04...0xff07 => self.timer.read_reg(addr),
            0xff0f => self.if_register,
            0xff10...0xff3f => self.apu.read_reg(addr),
            0xff40...0xff4f => self.gpu.read_reg(addr),
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.read_byte(addr - HIGH_RAM_START),
            0xffff => self.ie_register,

            // Unused addresses return 0xff for reads
            _ => 0xff,
        }
    }
}
