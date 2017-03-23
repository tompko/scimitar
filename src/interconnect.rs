use std::collections::HashMap;
use mem_map::*;
use cartridge::Cartridge;
use memory::Memory;
use gpu::Gpu;
use device::Device;
use apu::Apu;
use timer::Timer;

pub trait Interconnect {
    fn read_byte(&self, addr: u16) -> u8;
    fn read_halfword(&self, addr: u16) -> u16;

    fn write_byte(&mut self, addr: u16, val: u8);
    fn write_halfword(&mut self, addr: u16, val: u16);

    fn step(&mut self, cycles: u16, device: &mut Device);

    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub struct GBInterconnect {
    cartridge: Cartridge,
    gpu: Gpu,
    apu: Apu,
    timer: Timer,

    internal_ram: Memory,
    high_ram: Memory,
    if_register: u8,
    ie_register: u8,
}

pub struct MockInterconnect {
    memory: HashMap<u16, u8>,
}

impl GBInterconnect {
    pub fn new(cartridge: Cartridge) -> GBInterconnect {
        GBInterconnect {
            cartridge: cartridge,
            gpu: Gpu::new(),
            apu: Apu::new(),
            timer: Timer::default(),

            internal_ram: Memory::new(INTERNAL_RAM_LENGTH),
            high_ram: Memory::new(HIGH_RAM_END),

            if_register: 0,
            ie_register: 0,
        }
    }
}

impl Interconnect for GBInterconnect {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM0_START...ROM0_END => self.cartridge.read_byte(addr - ROM0_START),
            VRAM_START...VRAM_END => self.gpu.read_vram(addr - VRAM_START),
            INTERNAL_RAM_START...INTERNAL_RAM_END => {
                self.internal_ram.read_byte(addr - INTERNAL_RAM_START)
            }
            IRAM_ECHO_START...IRAM_ECHO_END => self.internal_ram.read_byte(addr - IRAM_ECHO_START),
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.read_byte(addr - HIGH_RAM_START),
            OAM_START...OAM_END => self.gpu.read_oam(addr - OAM_START),
            0xff04...0xff07 => self.timer.read_reg(addr),
            0xff0f => self.if_register,
            0xff10...0xff3f => self.apu.read_reg(addr),
            0xff40...0xff4b => self.gpu.read_reg(addr),
            0xffff => self.ie_register,
            _ => panic!("Read from unrecognized memory segment {:04x}", addr),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            VRAM_START...VRAM_END => self.gpu.write_vram(addr - VRAM_START, val),
            INTERNAL_RAM_START...INTERNAL_RAM_END => {
                self.internal_ram.write_byte(addr - INTERNAL_RAM_START, val)
            }
            IRAM_ECHO_START...IRAM_ECHO_END => {
                self.internal_ram.write_byte(addr - IRAM_ECHO_START, val)
            }
            HIGH_RAM_START...HIGH_RAM_END => self.high_ram.write_byte(addr - HIGH_RAM_START, val),
            OAM_START...OAM_END => self.gpu.write_oam(addr - OAM_START, val),
            0xff04...0xff07 => self.timer.write_reg(addr, val),
            0xff0f => self.if_register = val,
            0xff10...0xff3f => self.apu.write_reg(addr, val),
            0xff40...0xff4b => self.gpu.write_reg(addr, val),
            0xff50 => self.cartridge.disable_boot_rom(),
            0xffff => self.ie_register = val,
            _ => {
                panic!("Write to unrecognized memory segment {:04x} = {:02x}",
                       addr,
                       val)
            }
        }
    }

    fn read_halfword(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr + 1);

        ((msb as u16) << 8) | (lsb as u16)
    }

    fn write_halfword(&mut self, addr: u16, val: u16) {
        let lsb = (val & 0xff) as u8;
        let msb = (val >> 8) as u8;

        self.write_byte(addr, lsb);
        self.write_byte(addr + 1, msb);
    }

    fn step(&mut self, cycles: u16, device: &mut Device) {
        self.gpu.step(cycles, device);
        self.timer.step(cycles, device);
    }

    fn get_width(&self) -> usize {
        self.gpu.get_width()
    }

    fn get_height(&self) -> usize {
        self.gpu.get_height()
    }
}

impl MockInterconnect {
    pub fn new(instructions: &[u8]) -> MockInterconnect {
        let mut memory = HashMap::new();

        for (i, instr) in instructions.iter().enumerate() {
            memory.insert(i as u16, *instr);
        }

        MockInterconnect { memory: memory }
    }

    pub fn set_mem(&mut self, addr: u16, val: u8) {
        self.memory.insert(addr, val);
    }
}

impl Interconnect for MockInterconnect {
    fn read_byte(&self, addr: u16) -> u8 {
        *self.memory.get(&addr).unwrap_or(&0)
    }

    fn read_halfword(&self, addr: u16) -> u16 {
        let lsb = self.read_byte(addr);
        let msb = self.read_byte(addr + 1);

        ((msb as u16) << 8) | (lsb as u16)

    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        self.memory.insert(addr, val);
    }

    fn write_halfword(&mut self, addr: u16, val: u16) {
        let lsb = (val & 0xff) as u8;
        let msb = (val >> 8) as u8;

        self.write_byte(addr, lsb);
        self.write_byte(addr + 1, msb);
    }

    fn step(&mut self, _: u16, _: &mut Device) {}

    fn get_width(&self) -> usize {
        0
    }

    fn get_height(&self) -> usize {
        0
    }
}
