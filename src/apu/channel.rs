use apu::unit::*;
use apu::frame_sequencer::FrameSequencer;

pub struct Channel1 {
    pub sweep: Sweep,
    pub wave: SquareWave,
    pub length: LengthCounter,
    pub volume: VolumeEnvelope,
    pub timer: Timer,

    active: bool,
    dac_enabled: bool,
    pub length_enabled: bool,
}

pub struct Channel2 {
    pub wave: SquareWave,
    pub length: LengthCounter,
    pub volume: VolumeEnvelope,
    pub timer: Timer,

    active: bool,
    dac_enabled: bool,
    pub length_enabled: bool,
}

pub struct Channel3 {
    pub timer: Timer,
    pub wave: Wave,
    pub length: LengthCounter,
    pub volume: WaveVolume,

    active: bool,
    dac_enabled: bool,
    pub length_enabled: bool,
}

pub struct Channel4 {
    pub timer: Timer,
    pub lsfr: LSFR,
    pub length: LengthCounter,
    pub volume: VolumeEnvelope,

    active: bool,
    dac_enabled: bool,
    pub length_enabled: bool,
}

impl Default for Channel1 {
    fn default() -> Self {
        Channel1 {
            sweep: Sweep::default(),
            wave: SquareWave::default(),
            length: LengthCounter::new(6),
            volume: VolumeEnvelope::default(),
            timer: Timer::default(),

            active: false,
            dac_enabled: false,
            length_enabled: true,
        }
    }
}

impl Channel1 {
    pub fn trigger(&mut self, val: u8) {
        let trigger = ((val >> 7) & 0x01) != 0;

        if trigger {
            self.length.clocked = true;
            self.active = self.dac_enabled;
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_enabled = ((val >> 6) & 0x01) != 0;
    }

    pub fn write_volume(&mut self, val: u8) {
        self.volume.write(val);
        self.dac_enabled = (val >> 3) != 0;
        if !self.dac_enabled {
            self.active = false;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.dac_enabled = false;
        self.length_enabled = false;

        self.sweep.write(0);
        self.wave.write(0);
        self.length.write(0);
        self.volume.write(0);
        self.timer.write_lo(0);
        self.timer.write_hi(0);
    }

    pub fn step(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_step() && self.length_enabled {
            let fired = self.length.step();
            if fired {
                self.active = false;
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

impl Default for Channel2 {
    fn default() -> Self {
        Channel2 {
            wave: SquareWave::default(),
            length: LengthCounter::new(6),
            volume: VolumeEnvelope::default(),
            timer: Timer::default(),

            active: false,
            dac_enabled: false,
            length_enabled: true,
        }
    }
}

impl Channel2 {
    pub fn trigger(&mut self, val: u8) {
        let trigger = ((val >> 7) & 0x01) != 0;

        if trigger {
            self.length.clocked = true;
            self.active = self.dac_enabled;
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_enabled = ((val >> 6) & 0x01) != 0;
    }

    pub fn write_volume(&mut self, val: u8) {
        self.volume.write(val);
        self.dac_enabled = (val >> 3) != 0;
        if !self.dac_enabled {
            self.active = false;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.dac_enabled = false;
        self.length_enabled = false;

        self.wave.write(0);
        self.length.write(0);
        self.volume.write(0);
        self.timer.write_lo(0);
        self.timer.write_hi(0);
    }

    pub fn step(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_step() && self.length_enabled {
            let fired = self.length.step();
            if fired {
                self.active = false;
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

impl Default for Channel3 {
    fn default() -> Self {
        Channel3 {
            timer: Timer::default(),
            wave: Wave::default(),
            length: LengthCounter::new(8),
            volume: WaveVolume::default(),

            active: false,
            dac_enabled: false,
            length_enabled: true,
        }
    }
}

impl Channel3 {
    pub fn write_dac(&mut self, val: u8) {
        self.dac_enabled = ((val >> 7) & 0x01) != 0;
        if !self.dac_enabled {
            self.active = false
        }
    }

    pub fn trigger(&mut self, val: u8) {
        let trigger = ((val >> 7) & 0x01) != 0;

        if trigger {
            self.length.clocked = true;
            self.active = self.dac_enabled;
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_enabled = ((val >> 6) & 0x01) != 0;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.dac_enabled = false;
        self.length_enabled = false;

        self.timer.write_lo(0);
        self.timer.write_hi(0);
        self.length.write(0);
        self.volume.write(0);
    }

    pub fn step(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_step() && self.length_enabled {
            let fired = self.length.step();
            if fired {
                self.active = false;
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn dac_enabled(&self) -> bool {
        self.dac_enabled
    }
}

impl Default for Channel4 {
    fn default() -> Self {
        Channel4 {
            timer: Timer::default(),
            lsfr: LSFR::default(),
            length: LengthCounter::new(6),
            volume: VolumeEnvelope::default(),

            active: false,
            dac_enabled: false,
            length_enabled: true,
        }
    }
}

impl Channel4 {
    pub fn trigger(&mut self, val: u8) {
        let trigger = ((val >> 7) & 0x01) != 0;

        if trigger {
            self.length.clocked = true;
            self.active = self.dac_enabled;
        }
    }

    pub fn write_length_active(&mut self, val: u8) {
        self.length_enabled = ((val >> 6) & 0x01) != 0;
    }

    pub fn write_volume(&mut self, val: u8) {
        self.volume.write(val);
        self.dac_enabled = (val >> 3) != 0;
        if !self.dac_enabled {
            self.active = false;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.dac_enabled = false;
        self.length_enabled = false;

        self.timer.write_lo(0);
        self.timer.write_hi(0);
        self.lsfr.write(0);
        self.length.write(0);
        self.volume.write(0);
    }

    pub fn step(&mut self, frame_sequencer: &FrameSequencer) {
        if frame_sequencer.length_step() && self.length_enabled {
            let fired = self.length.step();
            if fired {
                self.active = false;
            }
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }
}
