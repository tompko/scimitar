use device::Device;

pub struct Gamepad {
    reg: u8,
}

impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            reg: 0,
        }
    }

    pub fn step(&mut self, _: u16, _: &mut Device) -> u8 {
        0
    }

    pub fn read_reg(&self) -> u8 {
        self.reg
    }

    pub fn write_reg(&mut self, val: u8) {
        self.reg = val
    }
}
