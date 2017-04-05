use device::{Device, Key};

pub struct Gamepad {
    p15: bool,
    p14: bool,

    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
}

impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            p15: false,
            p14: false,

            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    pub fn step(&mut self, _: u16, device: &mut Device) -> u8 {
        // TODO - allow for configuration
        self.up = device.key_down(Key::Up);
        self.down = device.key_down(Key::Down);
        self.left = device.key_down(Key::Left);
        self.right = device.key_down(Key::Right);
        self.a = device.key_down(Key::Z);
        self.b = device.key_down(Key::X);
        self.start = device.key_down(Key::Enter);
        self.select = device.key_down(Key::Backspace);

        // TODO - raise gamepad interrupt
        0
    }

    pub fn read_reg(&self) -> u8 {
        let mut ret = 0xc0;

        if !self.p15 {
            if !self.a {
                ret |= 1;
            }
            if !self.b {
                ret |= 1 << 1;
            }
            if !self.select {
                ret |= 1 << 2;
            }
            if !self.start {
                ret |= 1 << 3;
            }
        }
        if !self.p14 {
            if !self.right {
                ret |= 1;
            }
            if !self.left {
                ret |= 1 << 1;
            }
            if !self.up {
                ret |= 1 << 2;
            }
            if !self.down {
                ret |= 1 << 3;
            }
        }

        ret
    }

    pub fn write_reg(&mut self, val: u8) {
        self.p15 = val & (1 << 5) != 0;
        self.p14 = val & (1 << 4) != 0;
    }
}
