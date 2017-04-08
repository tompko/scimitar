use device::Device;

pub struct Timer {
    pub divider: u16,
    pub timer_counter: u8,
    pub timer_modulo: u8,

    timer_clock_select: u8,
    timer_enable: bool,

    tac_edge_delay: u8,
    tac_reload_delay: u8,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            divider: 0xabcc,
            timer_counter: 0,
            timer_modulo: 0,

            timer_clock_select: 0,
            timer_enable: false,

            tac_edge_delay: 0,
            tac_reload_delay: 0,
        }
    }
}

impl Timer {
    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xff04 => (self.divider >> 8) as u8,
            0xff05 => self.timer_counter,
            0xff06 => self.timer_modulo,
            0xff07 => self.timer_control(),
            _ => panic!("Read from non-timer register in timer {:04x}", addr),
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        match addr {
            0xff04 => self.divider = 0,
            0xff05 => {
                self.timer_counter = val;
                self.tac_reload_delay = 0;
            }
            0xff06 => self.timer_modulo = val,
            0xff07 => self.set_timer_control(val),
            _ => {
                panic!("Write to non-timer register in timer {:04x} = {:02x}",
                       addr,
                       val)
            }
        };
        self.divider_change();
    }

    pub fn step(&mut self, cycles: u16, _: &mut Device) -> u8 {
        let mut ret = 0;
        for _ in 0..cycles {
            if self.tac_reload_delay > 0 {
                self.tac_reload_delay -= 1;
                if self.tac_reload_delay == 0 {
                    self.timer_counter = self.timer_modulo;
                }
            }
            self.divider = self.divider.wrapping_add(1);

            let interrupt = self.divider_change();
            if interrupt {
                ret |= 0x01 << 2;
            }
        }

        ret
    }

    pub fn timer_control(&self) -> u8 {
        let enabled = if self.timer_enable { 1 } else { 0 };

        self.timer_clock_select | (enabled << 2)
    }

    fn divider_change(&mut self) -> bool {
        let new_delay = if !self.timer_enable {
            0
        } else {
            match self.timer_clock_select {
                0 => (self.divider >> 9) & 0x1,
                1 => (self.divider >> 3) & 0x1,
                2 => (self.divider >> 5) & 0x1,
                3 => (self.divider >> 7) & 0x1,
                _ => unreachable!(),
            }
        };

        let mut ret = false;
        if self.tac_edge_delay == 1 && new_delay == 0 {
            let (counter, overflow) = self.timer_counter.overflowing_add(1);
            self.timer_counter = counter;
            if overflow {
                ret = true;
                self.tac_reload_delay = 4;
            }
        }

        self.tac_edge_delay = new_delay as u8;
        ret
    }

    fn set_timer_control(&mut self, val: u8) {
        self.timer_enable = val & (1 << 2) != 0;
        self.timer_clock_select = val & 0x3;
    }
}
