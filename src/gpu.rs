use mem_map::*;

pub struct Gpu {
    vram: Box<[u8]>, // VRAM - mapped to 0x8000 - 0x9FFF
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            vram: vec![0; VRAM_LENGTH as usize].into_boxed_slice(),
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val;
    }
}
