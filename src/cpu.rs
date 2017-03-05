use interconnect::Interconnect;

#[derive(Clone)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        let mut ret = 0;
        if self.c {
            ret |= 1 << 4;
        }
        if self.h {
            ret |= 1 << 5;
        }
        if self.n {
            ret |= 1 << 6;
        }
        if self.z {
            ret |= 1 << 7;
        }
        ret
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags{
            z: (value & (1 << 7)) != 0,
            n: (value & (1 << 6)) != 0,
            h: (value & (1 << 5)) != 0,
            c: (value & (1 << 4)) != 0,
        }
    }
}

pub struct Cpu {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,

    pub instructions_to_di: u8,
    pub interrupts_enabled: bool,
}

static CYCLE_COUNTS: [u16; 256] = [
     4, 12,  8,  8,  4,  4,  8,  4, 20,  8,  8,  8,  4,  4,  8,  4,
     0, 12,  8,  8,  4,  4,  8,  4, 12,  8,  8,  8,  4,  4,  8,  4,
     8, 12,  8,  8,  4,  4,  8,  4,  8,  8,  8,  8,  4,  4,  8,  4,
     8, 12,  8,  8, 12, 12, 12,  4,  8,  8,  8,  8,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     8,  8,  8,  8,  8,  8,  0,  8,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     8, 12, 12, 16, 12, 16,  8, 16,  8, 16, 12,  0, 12, 24,  8, 16,
     8, 12, 12,  0, 12, 16,  8, 16,  8, 16, 12,  0, 12,  0,  8, 16,
    12, 12,  8,  0,  0, 16,  8, 16, 16,  4, 16,  0,  0,  0,  8, 16,
    12, 12,  8,  4,  0, 16,  8, 16, 12,  8, 16,  4,  0,  0,  8, 16
];

static CB_CYCLE_COUNTS: [u16; 256] = [
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8
];

impl Cpu {
    pub fn new() -> Cpu {
        let f = Flags{
            z: false,
            n: false,
            h: false,
            c: false,
        };

        Cpu{
            a: 0,
            f: f,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            sp: 0xfffe,
            pc: 0x100,

            instructions_to_di: 0,
            interrupts_enabled: true,
        }
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) -> u16 {
        let instr = self.read_pc_byte(interconnect);
        let mut cycle_count = CYCLE_COUNTS[instr as usize];

        match instr {
            0x00 => {
                // NOP - No Operation
            }
            0x01 => {
                // LD BC, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.b = msb;
                self.c = lsb;
            }
            0x06 => self.b = self.read_pc_byte(interconnect), // LD B,n
            0x0a => {
                let addr = self.bc();
                self.a = interconnect.read_byte(addr);
            }
            0x0e => self.c = self.read_pc_byte(interconnect), // LD C,n
            0x11 => {
                // LD DE, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.d = msb;
                self.e = lsb;
            }
            0x16 => self.d = self.read_pc_byte(interconnect), // LD D,n
            0x18 => {
                // JR n - realtive jump by n
                let n = self.read_pc_byte(interconnect);
                let old_pc = self.pc;
                self.pc = self.pc.wrapping_add(n as i8 as u16);
            }
            0x1a => self.a = interconnect.read_byte(self.de()),
            0x1e => self.e = self.read_pc_byte(interconnect), // LD E,n
            0x21 => {
                // LD HL, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.h = msb;
                self.l = lsb;
            }
            0x23 => {
                // INC HL
                let mut val = ((self.h as u16) << 8) | (self.l as u16);
                val = val.wrapping_add(1);
                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x26 => self.h = self.read_pc_byte(interconnect), // LD H,n
            0x2A => {
                // LDI A, (HL) - Load the value at address HL into A, increment HL
                let mut addr = ((self.h as u16) << 8) | (self.l as u16);
                self.a = interconnect.read_byte(addr);
                addr = addr.wrapping_add(1);
                self.h = (addr >> 8) as u8;
                self.l = (addr & 0xff) as u8;
            }
            0x2e => self.l = self.read_pc_byte(interconnect), // LD L,n
            0x31 => {
                // LD SP, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                let val = ((msb as u16) << 8) | lsb as u16;

                self.sp = val;
            }
            0x3c => {
                self.f.h = (self.a & 0xf).wrapping_add(1) > 0xf;
                self.a = self.a.wrapping_add(1);
                self.f.z = self.a == 0;
                self.f.n = false;
            }
            0x78 => self.a = self.b, // LD A, B
            0x79 => self.a = self.c, // LD A, C
            0x7a => self.a = self.d, // LD A, D
            0x7b => self.a = self.e, // LD A, E
            0x7c => self.a = self.h, // LD A, H
            0x7d => self.a = self.l, // LD A, L
            0x7e => {
                // LD A, (HL)
                let addr = self.hl();
                self.a = interconnect.read_byte(addr);
            }
            0x7f => {} // LD A, A
            0x3e => {
                // LD A, # - Load immediate 8-bit into A
                let val = self.read_pc_byte(interconnect);

                self.a = val;
            }
            0xc3 => {
                // JP nn - Jump to address nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.pc = ((msb as u16) << 8) | lsb as u16;
            }
            0xc9 => {
                // RET - pop return address and jump there
                let addr = self.pop(interconnect);
                self.pc = addr;
            }
            0xcd => {
                // CALL nn - Call function at nn
                let addr = interconnect.read_halfword(self.pc);
                self.pc += 2;

                let pc = self.pc;
                self.push(interconnect, pc);
                self.pc = addr;
            }
            0xe0 => {
                // LDH (n), A - Store A in memory 0xff00+n
                let n = self.read_pc_byte(interconnect);

                let addr = 0xff00 + (n as u16);
                interconnect.write_byte(addr, self.a);
            }
            0xe1 => {
                // POP HL
                let l = self.pop_byte(interconnect);
                let h = self.pop_byte(interconnect);

                self.h = h;
                self.l = l;
            }
            0xe5 => {
                // PUSH HL
                let h = self.h;
                let l = self.l;

                self.push_byte(interconnect, h);
                self.push_byte(interconnect, l);
            }
            0xea => {
                // LD nn, A - Store A to immediate address
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);
                let addr = ((msb as u16) << 8) | lsb as u16;

                interconnect.write_byte(addr, self.a);
            }
            0xf1 => {
                // POP AF
                let f = self.pop_byte(interconnect);
                let a = self.pop_byte(interconnect);

                self.a = a;
                self.f = f.into();
            }
            0xf3 => {
                // DI -Disable interrupts after the next instruction is executed
                self.instructions_to_di = 1;
            }
            0xf5 => {
                // PUSH AF
                let a = self.a;
                let f = self.f.clone();

                self.push_byte(interconnect, a);
                self.push_byte(interconnect, f.into());
            }
            0xfa => {
                let addr = self.read_pc_halfword(interconnect);
                self.a = interconnect.read_byte(addr);
            }
            _ => panic!("Unrecognized instruction {:02x}", instr),
        }

        if self.instructions_to_di > 0 {
            self.instructions_to_di -= 1;
            if self.instructions_to_di == 0 {
                self.disable_interrupts();
            }
        }

        cycle_count
    }

    fn read_pc_byte(&mut self, interconnect: &Interconnect) -> u8 {
        let val = interconnect.read_byte(self.pc);
        self.pc += 1;
        val
    }

    fn read_pc_halfword(&mut self, interconnect: &Interconnect) -> u16 {
        let lsb = self.read_pc_byte(interconnect);
        let msb = self.read_pc_byte(interconnect);

        ((msb as u16) << 8) | (lsb as u16)
    }

    fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    fn push(&mut self, interconnect: &mut Interconnect, addr: u16) {
        self.sp -= 1;
        interconnect.write_halfword(self.sp, addr);
        self.sp -= 1;
    }

    fn push_byte(&mut self, interconnect: &mut Interconnect, val: u8) {
        interconnect.write_byte(self.sp, val);
        self.sp += 1;
    }

    fn pop(&mut self, interconnect: &mut Interconnect) -> u16 {
        self.sp += 1;
        let ret = interconnect.read_halfword(self.sp);
        self.sp += 1;
        ret
    }

    fn pop_byte(&mut self, interconnect: &mut Interconnect) -> u8 {
        self.sp += 1;
        interconnect.read_byte(self.sp)
    }

    fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
}
