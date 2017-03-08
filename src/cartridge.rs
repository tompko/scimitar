use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

pub struct Cartridge {
    bytes: Box<[u8]>,
}

impl Cartridge {
    pub fn load<P: AsRef<Path>>(file_name: P) -> io::Result<Cartridge> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(Cartridge::from_bytes(&buffer))
    }

    pub fn from_bytes(bytes: &[u8]) -> Cartridge {
        let bytes_copy = bytes.to_vec();

        Cartridge { bytes: bytes_copy.into_boxed_slice() }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }
}
