use device::{Device, Key};
use interrupt::{Irq, Interrupt};

pub struct KeyPad {
    key_code: Key,
    pressed: bool,
}

impl KeyPad {
    fn new(key_code: Key) -> Self {
        KeyPad {
            key_code: key_code,
            pressed: false,
        }
    }

    fn step(&mut self, device: &mut Device, irq: &mut Irq) {
        let new_pressed = device.key_down(self.key_code);
        if !self.pressed && new_pressed {
            irq.raise_interrupt(Interrupt::Gamepad);
        }

        self.pressed = new_pressed;
    }

    fn pressed(&self) -> bool {
        self.pressed
    }
}

pub struct Gamepad {
    p15: bool,
    p14: bool,

    up: KeyPad,
    down: KeyPad,
    left: KeyPad,
    right: KeyPad,
    a: KeyPad,
    b: KeyPad,
    start: KeyPad,
    select: KeyPad,
}

impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            p15: false,
            p14: false,

            // TODO - allow for configuration
            up: KeyPad::new(Key::Up),
            down: KeyPad::new(Key::Down),
            left: KeyPad::new(Key::Left),
            right: KeyPad::new(Key::Right),
            a: KeyPad::new(Key::Z),
            b: KeyPad::new(Key::X),
            start: KeyPad::new(Key::Enter),
            select: KeyPad::new(Key::Backspace),
        }
    }

    pub fn step(&mut self, _: u16, device: &mut Device, irq: &mut Irq) {
        self.up.step(device, irq);
        self.down.step(device, irq);
        self.left.step(device, irq);
        self.right.step(device, irq);
        self.a.step(device, irq);
        self.b.step(device, irq);
        self.start.step(device, irq);
        self.select.step(device, irq);
    }

    pub fn read_reg(&self) -> u8 {
        let mut ret = 0xc0;

        if !self.p15 {
            if !self.a.pressed() {
                ret |= 1;
            }
            if !self.b.pressed() {
                ret |= 1 << 1;
            }
            if !self.select.pressed() {
                ret |= 1 << 2;
            }
            if !self.start.pressed() {
                ret |= 1 << 3;
            }
        }
        if !self.p14 {
            if !self.right.pressed() {
                ret |= 1;
            }
            if !self.left.pressed() {
                ret |= 1 << 1;
            }
            if !self.up.pressed() {
                ret |= 1 << 2;
            }
            if !self.down.pressed() {
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
