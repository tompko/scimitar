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

        let bytes_box = buffer.into_boxed_slice();

        Ok(Cartridge{
            bytes: bytes_box,
        })
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }
}
