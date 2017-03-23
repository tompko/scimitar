use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

pub struct Cartridge {
    bytes: Box<[u8]>,
    boot_rom: Box<[u8]>,

    boot_rom_active: bool,
}

impl Cartridge {
    pub fn load<P: AsRef<Path>>(file_name: P) -> io::Result<Cartridge> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(Cartridge::from_bytes(&buffer))
    }

    pub fn load_boot_rom<P: AsRef<Path>>(&mut self, file_name: P) -> io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.boot_rom = buffer.into_boxed_slice();
        self.boot_rom_active = true;

        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Cartridge {
        let bytes_copy = bytes.to_vec();

        Cartridge {
            bytes: bytes_copy.into_boxed_slice(),
            boot_rom: Box::default(),

            boot_rom_active: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        if self.boot_rom_active && addr < self.boot_rom.len() {
            self.boot_rom[addr]
        } else {
            self.bytes[addr]
        }
    }

    pub  fn disable_boot_rom(&mut self) {
        self.boot_rom_active = false;
    }
}
