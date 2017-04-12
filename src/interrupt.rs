pub enum Interrupt {
    VBlank,
    Stat,
    Timer,
    SerialIO,
    Gamepad
}

#[derive(Default)]
pub struct Irq {
    iflags: u8,
}

impl Irq {
    pub fn raise_interrupt(&mut self, int: Interrupt) {
        self.iflags |= match int {
            Interrupt::VBlank => 0x01,
            Interrupt::Stat => 0x02,
            Interrupt::Timer => 0x04,
            Interrupt::SerialIO => 0x08,
            Interrupt::Gamepad => 0x10,
        }
    }

    pub fn get_if(&self) -> u8 {
        (1 << 7) | (1 << 6) | (1 << 5) | self.iflags
    }
}
