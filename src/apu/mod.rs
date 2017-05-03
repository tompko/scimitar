use super::interrupt::Irq;
use super::device::Device;

mod channel;
mod unit;
mod frame_sequencer;

const AUDIO_STEP_CYCLE_COUNT: u16 = 8 * 1024; // Divides the main 4MHz to get 512Hz

pub struct Apu {
    chan1: channel::Channel1,
    chan2: channel::Channel2,
    chan3: channel::Channel3,
    chan4: channel::Channel4,

    out_chan_control: u8,
    output_terminal: u8,
    sound_active: bool,

    cycles: u16,
    frame_sequencer: frame_sequencer::FrameSequencer,
}

impl Apu {
    pub fn new() -> Self {
        Apu {
            chan1: channel::Channel1::default(),
            chan2: channel::Channel2::default(),
            chan3: channel::Channel3::default(),
            chan4: channel::Channel4::default(),

            out_chan_control: 0,
            output_terminal: 0,
            sound_active: false,

            cycles: 0,
            frame_sequencer: frame_sequencer::FrameSequencer::default(),
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.chan1.sweep.read(),
            0xff11 => self.chan1.wave.read() | 0x3f,
            0xff12 => self.chan1.volume.read(),
            0xff13 => 0xff,
            0xff14 => if self.chan1.length.clocked { 0xff } else { 0xbf },

            0xff16 => self.chan2.wave.read() | 0x3f,
            0xff17 => self.chan2.volume.read(),
            0xff18 => 0xff,
            0xff19 => if self.chan2.length.clocked { 0xff } else { 0xbf },

            0xff1a => if self.chan3.active { 0xff } else { 0x7f },
            0xff1b => 0xff,
            0xff1c => self.chan3.volume.read(),
            0xff1d => 0xff,
            0xff1e => if self.chan3.length.clocked { 0xff } else { 0xbf },

            0xff20 => 0xff,
            0xff21 => self.chan4.volume.read(),
            0xff22 => self.chan4.lsfr.read(),
            0xff23 => if self.chan4.length.clocked { 0xff } else { 0xbf },

            0xff24 => self.out_chan_control,
            0xff25 => self.output_terminal,
            0xff26 => {
                let high = if self.sound_active { 0xf0 } else { 0x70 };
                let chan1 = if self.chan1.active() { 0x01 } else { 0x00 };

                high | chan1
            },
            0xff30...0xff3f => self.chan3.wave.data[(addr - 0xff30) as usize],

            _ => 0xff,
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        if !self.sound_active && addr != 0xff26 {
            return
        }

        match addr {
            0xff10 => self.chan1.sweep.write(val),
            0xff11 => {
                self.chan1.wave.write(val);
                self.chan1.length.write(val);
            }
            0xff12 => self.chan1.volume.write(val),
            0xff13 => self.chan1.timer.write_lo(val),
            0xff14 => {
                self.chan1.timer.write_hi(val);
                self.chan1.write_reset(val);
                self.chan1.write_length_active(val);
            }

            0xff16 => {
                self.chan2.wave.write(val);
                self.chan2.length.write(val);
            }
            0xff17 => self.chan2.volume.write(val),
            0xff18 => self.chan2.timer.write_lo(val),
            0xff19 => {
                self.chan2.timer.write_hi(val);
                self.chan2.write_reset(val);
                self.chan2.write_length_active(val);
            }

            0xff1a => self.chan3.write_active(val),
            0xff1b => self.chan3.length.write(val),
            0xff1c => self.chan3.volume.write(val),
            0xff1d => self.chan3.timer.write_lo(val),
            0xff1e => {
                self.chan3.timer.write_hi(val);
                self.chan3.write_reset(val);
                self.chan3.write_length_active(val);
            }

            0xff20 => self.chan4.length.write(val),
            0xff21 => self.chan4.volume.write(val),
            0xff22 => self.chan4.lsfr.write(val),
            0xff23 => {
                self.chan4.write_reset(val);
                self.chan4.write_length_active(val);
            }

            0xff24 => self.out_chan_control = val,
            0xff25 => self.output_terminal = val,
            0xff26 => {
                self.sound_active = ((val >> 7) & 0x01) != 0;
                if !self.sound_active {
                    self.chan1.deactivate();
                    self.chan2.deactivate();
                    self.chan3.deactivate();
                    self.chan4.deactivate();

                    self.out_chan_control = 0;
                    self.output_terminal = 0;
                }
            }
            0xff30...0xff3f => self.chan3.wave.data[(addr - 0xff30) as usize] = val,

            _ => {},
        }
    }

    pub fn step(&mut self, cycles: u16, device: &mut Device, _: &mut Irq) {
        self.cycles += cycles;

        while self.cycles > AUDIO_STEP_CYCLE_COUNT {
            self.inner_step(device);
            self.cycles -= AUDIO_STEP_CYCLE_COUNT;
        }
    }

    fn inner_step(&mut self, device: &mut Device) {
        self.frame_sequencer.step();

        self.chan1.step(&self.frame_sequencer);
        self.chan2.step(&self.frame_sequencer);
        self.chan3.step(&self.frame_sequencer);
        self.chan4.step(&self.frame_sequencer);
    }
}
