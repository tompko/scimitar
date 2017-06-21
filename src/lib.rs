extern crate strfmt;
extern crate time;
extern crate combine;

pub mod vm;
pub mod bootrom;
pub mod cartridge;
pub mod config;
pub mod cpu;
pub mod interconnect;
pub mod device;
pub mod symbols;

mod mem_map;
mod memory;
mod ppu;
mod apu;
mod timer;
mod opcodes;
mod command;
mod gamepad;
mod interrupt;
mod events;

use std::path::Path;
use self::vm::VM;
use self::bootrom::Bootrom;
use self::cartridge::Cartridge;
use self::interconnect::Interconnect;
use self::symbols::Symbols;

#[derive(Default)]
pub struct Gameboy {
    cartridge: Option<String>,
    boot_rom: Option<String>,
    symbols: Option<String>,
    start_in_debug: bool,
}

impl Gameboy {
    pub fn with_cartridge(mut self, cart: Option<&str>) -> Self {
        self.cartridge = match cart {
            Some(s) => Some(s.to_owned()),
            None => None,
        };

        self
    }

    pub fn with_boot_rom(mut self, boot_rom: Option<&str>) -> Self {
        self.boot_rom = match boot_rom {
            Some(s) => Some(s.to_owned()),
            None => None,
        };
        self
    }

    pub fn with_symbols(mut self, sym: Option<&str>) -> Self {
        self.symbols = match sym {
            Some(s) => Some(s.to_owned()),
            None => None,
        };
        self
    }

    pub fn start_in_debug(mut self, sid: bool) -> Self {
        self.start_in_debug = sid;
        self
    }

    pub fn build(self) -> vm::VM {
        let input_file = self.cartridge.unwrap();
        let boot_rom_file = self.boot_rom.unwrap();
        let cartridge = Cartridge::load(Path::new(&input_file)).unwrap();
        let start_in_debug = self.start_in_debug;

        let boot_rom = Bootrom::load(Path::new(&boot_rom_file)).unwrap();

        let symbols = if let Some(sym_file) = self.symbols {
            Symbols::load(sym_file).unwrap()
        } else {
            Symbols::default()
        };

        let interconnect = Interconnect::new(boot_rom, cartridge);

        VM::new(interconnect, start_in_debug, symbols)
    }
}
