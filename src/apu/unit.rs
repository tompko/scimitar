#[derive(Default)]
pub struct Sweep {
    sweep_time: u8,
    sweep_increase: bool,
    sweep_shift: u8,
}

impl Sweep {
    pub fn read(&self) -> u8 {
        0x80 | (self.sweep_time << 4) | if self.sweep_increase { 1 << 3 } else { 0 } | self.sweep_shift
    }

    pub fn write(&mut self, val: u8) {
        self.sweep_time = (val >> 4 ) & 0x07;
        self.sweep_increase = ((val >> 3) & 0x01) != 0;
        self.sweep_shift = val & 0x7;
    }
}

#[derive(Default)]
pub struct SquareWave {
    duty_pattern: u8,
}

impl SquareWave {
    pub fn read(&self) -> u8 {
        self.duty_pattern << 6
    }

    pub fn write(&mut self, val: u8) {
        self.duty_pattern = (val >> 6) & 0x03;
    }
}

pub struct LengthCounter {
    length: u8,
    mask: u8,
}

impl LengthCounter {
    pub fn new(mask: u8) -> Self {
        LengthCounter {
            length: 0,
            mask: mask
        }
    }

    // pub fn read(&self) -> u8 {
    //     self.length
    // }

    pub fn write(&mut self, val: u8) {
        self.length = val & self.mask;
    }
}

#[derive(Default)]
pub struct VolumeEnvelope {
    volume: u8,
    volume_increase: bool,
    volume_delta: u8,
}

impl VolumeEnvelope {
    pub fn read(&self) -> u8 {
        (self.volume << 4) | if self.volume_increase { 1 << 3 } else { 0 } | self.volume_delta
    }

    pub fn write(&mut self, val: u8) {
        self.volume = val >> 4;
        self.volume_increase = ((val >> 3) & 0x01) != 0;
        self.volume_delta = val & 0x07;
    }
}

#[derive(Default)]
pub struct Timer {
    frequency: u16,
}

impl Timer {
    pub fn write_lo(&mut self, val: u8) {
        self.frequency = (self.frequency & 0xf0) | val as u16;
    }

    pub fn write_hi(&mut self, val: u8) {
        let val = (val & 0x05) as u16;
        self.frequency = (self.frequency & 0x0f) | (val << 8)
    }
}

#[derive(Default)]
pub struct Wave {
    pub data: [u8; 16],
}

#[derive(Default)]
pub struct WaveVolume {
    volume: u8,
}

impl WaveVolume {
    pub fn write(&mut self, val: u8) {
        self.volume = (val >> 5) & 0x03;
    }

    pub fn read(&self) -> u8 {
        0x80 | (self.volume << 5) | 0x1f
    }
}

#[derive(Default)]
pub struct LSFR {
    shift_clock: u8,
    width: u8,
    divider: u8,
}

impl LSFR {
    pub fn write(&mut self, val: u8) {
        self.shift_clock = val >> 4;
        self.width = (val >> 3) & 0x01;
        self.divider = val & 0x07;
    }

    pub fn read(&self) -> u8 {
        (self.shift_clock << 4) | (self.width << 3) | self.divider
    }
}
