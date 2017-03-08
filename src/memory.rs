pub struct Memory {
    mem: Box<[u8]>,
}

impl Memory {
    pub fn new(size: u16) -> Memory {
        Memory { mem: vec![0; size as usize].into_boxed_slice() }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }
}
