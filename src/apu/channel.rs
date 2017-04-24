use apu::unit::*;

pub struct Channel1 {
    pub sweep: Sweep,
    pub wave: SquareWave,
    pub length: LengthCounter,
    pub volume: VolumeEnvelope,
    pub timer: Timer,
    pub length_active: bool,
}

pub struct Channel2 {
    pub wave: SquareWave,
    pub length: LengthCounter,
    pub volume: VolumeEnvelope,
    pub timer: Timer,
    pub length_active: bool,
}

pub struct Channel3 {
    pub active: bool,
    pub timer: Timer,
    pub wave: Wave,
    pub length: LengthCounter,
    pub volume: WaveVolume,
    pub length_active: bool,
}

pub struct Channel4 {
    pub timer: Timer,
    pub lsfr: LSFR,
    pub length: LengthCounter,
    pub volume: VolumeEnvelope,
    pub length_active: bool,
}

impl Default for Channel1 {
    fn default() -> Self {
        Channel1 {
            sweep: Sweep::default(),
            wave: SquareWave::default(),
            length: LengthCounter::new(0x3f),
            volume: VolumeEnvelope::default(),
            timer: Timer::default(),
            length_active: false,
        }
    }
}

impl Channel1 {
    pub fn write_reset(&mut self, val: u8) {
        let reset = ((val >> 7) & 0x01) != 0;

        if reset {
            println!("WARN: Sound reset not implemented");
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_active = ((val >> 6) & 0x01) != 0;
    }

    pub fn deactivate(&mut self) {
        self.sweep.write(0);
        self.wave.write(0);
        self.length.write(0);
        self.volume.write(0);
        self.timer.write_lo(0);
        self.timer.write_hi(0);
        self.length_active = false;
    }
}

impl Default for Channel2 {
    fn default() -> Self {
        Channel2 {
            wave: SquareWave::default(),
            length: LengthCounter::new(0x3f),
            volume: VolumeEnvelope::default(),
            timer: Timer::default(),
            length_active: false,
        }
    }
}

impl Channel2 {
    pub fn write_reset(&mut self, val: u8) {
        let reset = ((val >> 7) & 0x01) != 0;

        if reset {
            println!("WARN: Sound reset not implemented");
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_active = ((val >> 6) & 0x01) != 0;
    }

    pub fn deactivate(&mut self) {
        self.wave.write(0);
        self.length.write(0);
        self.volume.write(0);
        self.timer.write_lo(0);
        self.timer.write_hi(0);
        self.length_active = false;
    }
}

impl Default for Channel3 {
    fn default() -> Self {
        Channel3 {
            active: false,
            timer: Timer::default(),
            wave: Wave::default(),
            length: LengthCounter::new(0xff),
            volume: WaveVolume::default(),
            length_active: false,
        }
    }
}

impl Channel3 {
    pub fn write_active(&mut self, val: u8) {
        self.active = ((val >> 7) & 0x01) != 0;
    }

    pub fn write_reset(&mut self, val: u8) {
        let reset = ((val >> 7) & 0x01) != 0;

        if reset {
            println!("WARN: Sound reset not implemented");
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_active = ((val >> 6) & 0x01) != 0;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.timer.write_lo(0);
        self.timer.write_hi(0);
        self.length.write(0);
        self.volume.write(0);
        self.length_active = false;
    }
}

impl Default for Channel4 {
    fn default() -> Self {
        Channel4 {
            timer: Timer::default(),
            lsfr: LSFR::default(),
            length: LengthCounter::new(0x3f),
            volume: VolumeEnvelope::default(),
            length_active: false,
        }
    }
}

impl Channel4 {
    pub fn write_reset(&mut self, val: u8) {
        let reset = ((val >> 7) & 0x01) != 0;

        if reset {
            println!("WARN: Sound reset not implemented");
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_active = ((val >> 6) & 0x01) != 0;
    }

    pub fn deactivate(&mut self) {
        self.timer.write_lo(0);
        self.timer.write_hi(0);
        self.lsfr.write(0);
        self.length.write(0);
        self.volume.write(0);
        self.length_active = false;
    }
}
