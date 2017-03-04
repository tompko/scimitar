use mem_map::*;
use cartridge::Cartridge;

pub struct Interconnect {
    cartridge: Cartridge,
}

impl Interconnect {
    pub fn new(cartridge: Cartridge) -> Interconnect {
        Interconnect{
            cartridge: cartridge,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            ROM0_START...ROM0_END => self.cartridge.read_byte(addr - ROM0_START),
            _ => panic!("Read from unrecognized memory segment {:04x}", addr),
        }
    }
}
