use std::collections::HashMap;
use mem_map::*;
use cartridge::Cartridge;
use memory::Memory;

pub trait Interconnect {
    fn read_byte(&self, addr: u16) ->u8;
    fn read_halfword(&self, addr: u16) -> u16;

    fn write_byte(&mut self, addr: u16, val: u8);
    fn write_halfword(&mut self, addr: u16, val: u16);
}

pub struct GBInterconnect {
    cartridge: Cartridge,
    internal_ram: Memory,
    io_ports: Memory,
    ie_register: u8,
}

pub struct MockInterconnect {
    memory: HashMap<u16, u8>,
}

impl GBInterconnect {
    pub fn new(cartridge: Cartridge) -> GBInterconnect {
        GBInterconnect{
            cartridge: cartridge,
            internal_ram: Memory::new(INTERNAL_RAM_SIZE),
            io_ports: Memory::new(IO_PORTS_SIZE),
            ie_register: 0,
        }
    }
}

impl Interconnect for GBInterconnect {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM0_START...ROM0_END => self.cartridge.read_byte(addr - ROM0_START),
            INTERNAL_RAM_START...INTERNAL_RAM_END => self.internal_ram.read_byte(addr - INTERNAL_RAM_START),
            IRAM_ECHO_START...IRAM_ECHO_END => self.internal_ram.read_byte(addr - IRAM_ECHO_START),
            IO_PORTS_START...IO_PORTS_END => self.io_ports.read_byte(addr - IO_PORTS_START),
            _ => panic!("Read from unrecognized memory segment {:04x}", addr),
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            INTERNAL_RAM_START...INTERNAL_RAM_END => self.internal_ram.write_byte(addr - INTERNAL_RAM_START, val),
            IRAM_ECHO_START...IRAM_ECHO_END => self.internal_ram.write_byte(addr - IRAM_ECHO_START, val),
            IO_PORTS_START...IO_PORTS_END => self.io_ports.write_byte(addr - IO_PORTS_START, val),
            0xffff => self.ie_register = val,
            _ => panic!("Write to unrecognized memory segment {:04x} = {:02x}", addr, val),
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
}

impl MockInterconnect {
    pub fn new(instructions: &[u8]) -> MockInterconnect {
        let mut memory = HashMap::new();

        for (i, instr) in instructions.iter().enumerate() {
            memory.insert(i as u16, *instr);
        }

        MockInterconnect{
            memory: memory,
        }
    }

    pub fn set_mem(&mut self, addr: u16, val: u8) {
        self.memory.insert(addr, val);
    }
}

impl Interconnect for MockInterconnect {
    fn read_byte(&self, addr: u16) -> u8 {
        let val = *self.memory.get(&addr).unwrap_or(&0);
        val
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
        self.write_byte(addr + 1, lsb);
    }
}
