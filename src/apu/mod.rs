pub struct Apu {
    chan1_sweep: u8, // 0xff10 - Channel 1 Sweep Register
    chan1_pattern: u8, // 0xff11 - Channel 1 Sound length/Wave duty register
    chan1_volume: u8,
    chan1_freq_lo: u8,
    chan1_freq_hi: u8,

    chan2_pattern: u8,
    chan2_volume: u8,
    chan2_freq_lo: u8,
    chan2_freq_hi: u8,

    chan3_active: u8,
    chan3_length: u8,
    chan3_out_level: u8,
    chan3_freq_lo: u8,
    chan3_freq_hi: u8,
    chan3_data: [u8; 16],

    chan4_length: u8,
    chan4_volume: u8,
    chan4_counter: u8,
    chan4_control: u8,

    out_chan_control: u8,
    output_terminal: u8,
    sound_active: u8,
}

impl Apu {
    pub fn new() -> Self {
        Apu {
            chan1_sweep: 0,
            chan1_pattern: 0,
            chan1_volume: 0,
            chan1_freq_lo: 0,
            chan1_freq_hi: 0,

            chan2_pattern: 0,
            chan2_volume: 0,
            chan2_freq_lo: 0,
            chan2_freq_hi: 0,

            chan3_active: 0,
            chan3_length: 0,
            chan3_out_level: 0,
            chan3_freq_lo: 0,
            chan3_freq_hi: 0,
            chan3_data: [0; 16],

            chan4_length: 0,
            chan4_volume: 0,
            chan4_counter: 0,
            chan4_control: 0,

            out_chan_control: 0,
            output_terminal: 0,
            sound_active: 0,
        }
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xff10 => self.chan1_sweep,
            0xff11 => self.chan1_pattern,
            0xff12 => self.chan1_volume,
            0xff13 => self.chan1_freq_lo,
            0xff14 => self.chan1_freq_hi,
            0xff16 => self.chan2_pattern,
            0xff17 => self.chan2_volume,
            0xff18 => self.chan2_freq_lo,
            0xff19 => self.chan2_freq_hi,
            0xff1a => self.chan3_active,
            0xff1b => self.chan3_length,
            0xff1c => self.chan3_out_level,
            0xff1d => self.chan3_freq_lo,
            0xff1e => self.chan3_freq_hi,
            0xff20 => self.chan4_length,
            0xff21 => self.chan4_volume,
            0xff22 => self.chan4_counter,
            0xff23 => self.chan4_control,
            0xff24 => self.out_chan_control,
            0xff25 => self.output_terminal,
            0xff26 => self.sound_active,
            0xff30...0xff3f => self.chan3_data[(addr - 0xff30) as usize],
            _ => panic!("Read from non-apu register in apu {:04x}", addr),
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        match addr {
            0xff10 => self.chan1_sweep = val,
            0xff11 => self.chan1_pattern = val,
            0xff12 => self.chan1_volume = val,
            0xff13 => self.chan1_freq_lo = val,
            0xff14 => self.chan1_freq_hi = val,
            0xff16 => self.chan2_pattern = val,
            0xff17 => self.chan2_volume = val,
            0xff18 => self.chan2_freq_lo = val,
            0xff19 => self.chan2_freq_hi = val,
            0xff1a => self.chan3_active = val,
            0xff1b => self.chan3_length = val,
            0xff1c => self.chan3_out_level = val,
            0xff1d => self.chan3_freq_lo = val,
            0xff1e => self.chan3_freq_hi = val,
            0xff20 => self.chan4_length = val,
            0xff21 => self.chan4_volume = val,
            0xff22 => self.chan4_counter = val,
            0xff23 => self.chan4_control = val,
            0xff24 => self.out_chan_control = val,
            0xff25 => self.output_terminal = val,
            0xff26 => self.sound_active = val,
            0xff30...0xff3f => self.chan3_data[(addr - 0xff30) as usize] = val,
            _ => {
                panic!("Write to non-apu register in apu {:04x} = {:02x}",
                       addr,
                       val)
            }
        }
    }
}
