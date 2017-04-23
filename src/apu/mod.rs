use super::interrupt::Irq;
use super::device::Device;

mod channel;
mod unit;

pub struct Apu {
    chan1: channel::Channel1,
    chan2: channel::Channel2,
    chan3: channel::Channel3,
    chan4: channel::Channel4,

    out_chan_control: u8,
    output_terminal: u8,
    sound_active: u8,
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
            sound_active: 0,
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.chan1.sweep.read(),
            0xff11 => self.chan1.wave.read() | 0x3f,
            0xff12 => self.chan1.volume.read(),
            0xff13 => 0xff,
            0xff14 => if self.chan1.length_active { 0xff } else { 0xbf },

            0xff16 => self.chan2.wave.read() | 0x3f,
            0xff17 => self.chan2.volume.read(),
            0xff18 => 0xff,
            0xff19 => if self.chan2.length_active { 0xff } else { 0xbf },

            0xff1a => if self.chan3.active { 0xff } else { 0x7f },
            0xff1b => 0xff,
            0xff1c => self.chan3.volume.read(),
            0xff1d => 0xff,
            0xff1e => if self.chan3.length_active { 0xff } else { 0xbf },

            0xff20 => 0xff,
            0xff21 => self.chan4.volume.read(),
            0xff22 => self.chan4.lsfr.read(),
            0xff23 => if self.chan4.length_active { 0xff } else { 0xbf },

            0xff24 => self.out_chan_control,
            0xff25 => self.output_terminal,
            0xff26 => self.sound_active,
            0xff30...0xff3f => self.chan3.wave.data[(addr - 0xff30) as usize],
            _ => 0xff,
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
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
            0xff26 => self.sound_active = val,
            0xff30...0xff3f => self.chan3.wave.data[(addr - 0xff30) as usize] = val,
            _ => {},
        }
    }

    // pub fn step(&mut self, cycles: u16, device: &mut Device, irq: &mut Irq) {
    pub fn step(&mut self, _: u16, _: &mut Device, _: &mut Irq) {
    }
}
