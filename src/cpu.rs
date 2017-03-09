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
        Flags {
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

#[cfg_attr(rustfmt, rustfmt_skip)]
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

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(dead_code)] // TODO - remove once CB instructions are implemented
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
        let f = Flags {
            z: false,
            n: false,
            h: false,
            c: false,
        };

        Cpu {
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

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    pub fn step(&mut self, interconnect: &mut Interconnect) -> u16 {
        let instr = self.read_pc_byte(interconnect);
        let cycle_count = CYCLE_COUNTS[instr as usize];

        match instr {
            0x00 => {} // NOP - No Operation
            0x01 => {
                // LD BC, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.b = msb;
                self.c = lsb;
            }
            0x02 => interconnect.write_byte(self.bc(), self.a), // LD (BC), A
            0x06 => self.b = self.read_pc_byte(interconnect), // LD B,n
            0x08 => {
                // LD (nn), SP
                let addr = self.read_pc_halfword(interconnect);

                interconnect.write_halfword(addr, self.sp);
            }
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
            0x12 => interconnect.write_byte(self.de(), self.a), // LD (DE), A
            0x16 => self.d = self.read_pc_byte(interconnect), // LD D,n
            0x18 => {
                // JR n - realtive jump by n
                let n = self.read_pc_byte(interconnect);
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
            0x22 => {
                // LDI (HL), A
                interconnect.write_byte(self.hl(), self.a);
                let val = self.hl().wrapping_add(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x23 => {
                // INC HL
                let mut val = ((self.h as u16) << 8) | (self.l as u16);
                val = val.wrapping_add(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x26 => self.h = self.read_pc_byte(interconnect), // LD H,n
            0x2a => {
                // LDI A, (HL) - Load the value at address HL into A, increment HL
                self.a = interconnect.read_byte(self.hl());
                let val = self.hl().wrapping_add(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x2e => self.l = self.read_pc_byte(interconnect), // LD L,n
            0x31 => {
                // LD SP, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                let val = ((msb as u16) << 8) | lsb as u16;

                self.sp = val;
            }
            0x32 => {
                // LDD (HL), A
                interconnect.write_byte(self.hl(), self.a);
                let val = self.hl().wrapping_sub(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x36 => {
                let val = self.read_pc_byte(interconnect);

                interconnect.write_byte(self.hl(), val);
            }
            0x37 => {
                // SCF
                self.f.n = false;
                self.f.h = false;
                self.f.c = true;
            }
            0x3a => {
                self.a = interconnect.read_byte(self.hl());
                let val = self.hl().wrapping_sub(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x3c => {
                self.f.h = (self.a & 0xf).wrapping_add(1) > 0xf;
                self.a = self.a.wrapping_add(1);
                self.f.z = self.a == 0;
                self.f.n = false;
            }
            0x3e => {
                // LD A, # - Load immediate 8-bit into A
                let val = self.read_pc_byte(interconnect);

                self.a = val;
            }
            0x3f => {
                // CCF - complement carry flag
                self.f.n = false;
                self.f.h = false;
                self.f.c = !self.f.c;
            }
            0x40 => {} // LD B, B
            0x41 => self.b = self.c, // LD B, C
            0x42 => self.b = self.d, // LD B, D
            0x43 => self.b = self.e, // LD B, E
            0x44 => self.b = self.h, // LD B, H
            0x45 => self.b = self.l, // LD B, L
            0x46 => self.b = interconnect.read_byte(self.hl()), // LD B, (HL)
            0x47 => self.b = self.a, // LD B, A
            0x48 => self.c = self.b, // LD C, B
            0x49 => {} // LD C, C
            0x4a => self.c = self.d, // LD C, D
            0x4b => self.c = self.e, // LD C, E
            0x4c => self.c = self.h, // LD C, H
            0x4d => self.c = self.l, // LD C, L
            0x4e => self.c = interconnect.read_byte(self.hl()), // LD C, (HL)
            0x4f => self.c = self.a, // LD C, A
            0x50 => self.d = self.b, // LD D, B
            0x51 => self.d = self.c, // LD D, C
            0x52 => {} // LD D, D
            0x53 => self.d = self.e, // LD D, E
            0x54 => self.d = self.h, // LD D, H
            0x55 => self.d = self.l, // LD D, L
            0x56 => self.d = interconnect.read_byte(self.hl()), // LD D, (HL)
            0x57 => self.d = self.a, // LD D, A
            0x58 => self.e = self.b, // LD E, B
            0x59 => self.e = self.c, // LD E, C
            0x5a => self.e = self.d, // LD E, D
            0x5b => {} // LD E, E
            0x5c => self.e = self.h, // LD E, H
            0x5d => self.e = self.l, // LD E, L
            0x5e => self.e = interconnect.read_byte(self.hl()), // LD E, (HL)
            0x5f => self.e = self.a, // LD E, A
            0x60 => self.h = self.b, // LD H, B
            0x61 => self.h = self.c, // LD H, C
            0x62 => self.h = self.d, // LD H, D
            0x63 => self.h = self.e, // LD H, E
            0x64 => {} // LD H, H
            0x65 => self.h = self.l, // LD H, L
            0x66 => self.h = interconnect.read_byte(self.hl()), // LD H, (HL)
            0x67 => self.h = self.a, // LD H, A
            0x68 => self.l = self.b, // LD L, B
            0x69 => self.l = self.c, // LD L, C
            0x6a => self.l = self.d, // LD L, D
            0x6b => self.l = self.e, // LD L, E
            0x6c => self.l = self.h, // LD L, H
            0x6f => self.l = self.a, // LD L, A
            0x6d => {} // LD L, L
            0x6e => self.l = interconnect.read_byte(self.hl()), // LD L, (HL)
            0x70 => interconnect.write_byte(self.hl(), self.b), // LD (HL), B
            0x71 => interconnect.write_byte(self.hl(), self.c), // LD (HL), C
            0x72 => interconnect.write_byte(self.hl(), self.d), // LD (HL), D
            0x73 => interconnect.write_byte(self.hl(), self.e), // LD (HL), E
            0x74 => interconnect.write_byte(self.hl(), self.h), // LD (HL), H
            0x75 => interconnect.write_byte(self.hl(), self.l), // LD (HL), L
            0x77 => interconnect.write_byte(self.hl(), self.a), // LD (HL), A
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
            0x80 => {
                // ADD A, B
                let val = self.b;
                self.a = self.addc_a(val, false);
            }
            0x81 => {
                // ADD A, C
                let val = self.c;
                self.a = self.addc_a(val, false);
            }
            0x82 => {
                // ADD A, D
                let val = self.d;
                self.a = self.addc_a(val, false);
            }
            0x83 => {
                // ADD A, E
                let val = self.e;
                self.a = self.addc_a(val, false);
            }
            0x84 => {
                // ADD A, H
                let val = self.h;
                self.a = self.addc_a(val, false);
            }
            0x85 => {
                // ADD A, L
                let val = self.l;
                self.a = self.addc_a(val, false);
            }
            0x86 => {
                // ADD A, (HL)
                let val = interconnect.read_byte(self.hl());
                self.a = self.addc_a(val, false);
            }
            0x87 => {
                // ADD A, A
                let val = self.a;
                self.a = self.addc_a(val, false);
            }
            0x88 => {
                // ADDC A, B
                let val = self.b;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x89 => {
                // ADDC A, C
                let val = self.c;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x8a => {
                // ADDC A, D
                let val = self.d;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x8b => {
                // ADDC A, E
                let val = self.e;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x8c => {
                // ADDC A, H
                let val = self.h;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x8d => {
                // ADDC A, L
                let val = self.l;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x8e => {
                // ADDC A, (HL)
                let val = interconnect.read_byte(self.hl());
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x8f => {
                // ADDC A, A
                let val = self.a;
                let carry = self.f.c;
                self.a = self.addc_a(val, carry);
            }
            0x90 => {
                // SUB A, B
                let val = self.b;
                self.a = self.subc_a(val, false);
            }
            0x91 => {
                // SUB A, C
                let val = self.c;
                self.a = self.subc_a(val, false);
            }
            0x92 => {
                // SUB A, D
                let val = self.d;
                self.a = self.subc_a(val, false);
            }
            0x93 => {
                // SUB A, E
                let val = self.e;
                self.a = self.subc_a(val, false);
            }
            0x94 => {
                // SUB A, H
                let val = self.h;
                self.a = self.subc_a(val, false);
            }
            0x95 => {
                // SUB A, L
                let val = self.l;
                self.a = self.subc_a(val, false);
            }
            0x96 => {
                // SUB A, (HL)
                let val = interconnect.read_byte(self.hl());
                self.a = self.subc_a(val, false);
            }
            0x97 => {
                // SUB A, A
                let val = self.a;
                self.a = self.subc_a(val, false);
            }
            0x98 => {
                // SUBC A, B
                let val = self.b;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x99 => {
                // SUBC A, C
                let val = self.c;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x9a => {
                // SUBC A, D
                let val = self.d;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x9b => {
                // SUBC A, E
                let val = self.e;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x9c => {
                // SUBC A, H
                let val = self.h;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x9d => {
                // SUBC A, L
                let val = self.l;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x9e => {
                // SUBC A, (HL)
                let val = interconnect.read_byte(self.hl());
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0x9f => {
                // SUBC A, A
                let val = self.a;
                let carry = self.f.c;
                self.a = self.subc_a(val, carry);
            }
            0xc1 => {
                // POP BC
                let c = self.pop_byte(interconnect);
                let b = self.pop_byte(interconnect);

                self.b = b;
                self.c = c;
            }
            0xc3 => {
                // JP nn - Jump to address nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.pc = ((msb as u16) << 8) | lsb as u16;
            }
            0xc5 => {
                // PUSH BC
                let halfword = self.bc();
                self.push_halfword(interconnect, halfword);
            }
            0xc6 => {
                let n = self.read_pc_byte(interconnect);
                self.a = self.addc_a(n, false);
            }
            0xc9 => {
                // RET - pop return address and jump there
                let addr = self.pop_halfword(interconnect);
                self.pc = addr;
            }
            0xcd => {
                // CALL nn - Call function at nn
                let addr = interconnect.read_halfword(self.pc);
                self.pc += 2;

                let pc = self.pc;
                self.push_halfword(interconnect, pc);
                self.pc = addr;
            }
            0xce => {
                // ADDC A, n
                let val = self.read_pc_byte(interconnect);
                let carry = self.f.c;

                self.a = self.addc_a(val, carry);
            }
            0xd1 => {
                // POP DE
                let e = self.pop_byte(interconnect);
                let d = self.pop_byte(interconnect);

                self.d = d;
                self.e = e;
            }
            0xd5 => {
                // PUSH DE
                let halfword = self.de();
                self.push_halfword(interconnect, halfword);
            }
            0xd6 => {
                // SUB A, n
                let n = self.read_pc_byte(interconnect);
                self.a = self.subc_a(n, false);
            }
            0xde => {
                // SUBC A, n
                let n = self.read_pc_byte(interconnect);
                let carry = self.f.c;
                self.a = self.subc_a(n, carry);
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
            0xe2 => {
                // LD (C), A
                let addr = 0xff00 + (self.c as u16);
                interconnect.write_byte(addr, self.a);
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
                let addr = self.read_pc_halfword(interconnect);
                interconnect.write_byte(addr, self.a);
            }
            0xf0 => {
                let n = self.read_pc_byte(interconnect);
                let addr = 0xff00 + (n as u16);

                self.a = interconnect.read_byte(addr);
            }
            0xf1 => {
                // POP AF
                let f = self.pop_byte(interconnect);
                let a = self.pop_byte(interconnect);

                self.a = a;
                self.f = f.into();
            }
            0xf2 => {
                let addr = 0xff00 + (self.c as u16);
                self.a = interconnect.read_byte(addr);
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
            0xf8 => {
                // LD HL, SP+n
                let n = self.read_pc_byte(interconnect) as u16;
                let addr = self.sp + n;
                self.h = (addr >> 8) as u8;
                self.l = (addr & 0xff) as u8;
            }
            0xf9 => self.sp = self.hl(), // LD SP, HL
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

    fn push_halfword(&mut self, interconnect: &mut Interconnect, addr: u16) {
        self.sp -= 2;
        interconnect.write_halfword(self.sp, addr);
    }

    fn push_byte(&mut self, interconnect: &mut Interconnect, val: u8) {
        println!("PUSH {:02x} {:04x}", val, self.sp);
        self.sp -= 1;
        interconnect.write_byte(self.sp, val);
    }

    fn pop_halfword(&mut self, interconnect: &mut Interconnect) -> u16 {
        let ret = interconnect.read_halfword(self.sp);
        self.sp += 2;
        ret
    }

    fn pop_byte(&mut self, interconnect: &mut Interconnect) -> u8 {
        let val = interconnect.read_byte(self.sp);
        self.sp += 1;
        val
    }

    fn addc_a(&mut self, val: u8, carry: bool) -> u8 {
        let carry = if carry { 1 } else { 0 };
        let (tmp, overflow) = self.a.overflowing_add(val);
        let (r, overflow_c) = tmp.overflowing_add(carry);

        self.f.z = r == 0;
        self.f.n = false;
        self.f.h = ((self.a & 0xf) + (val & 0xf) + carry) > 0xf;
        self.f.c = overflow || overflow_c;

        r
    }

    fn subc_a(&mut self, val: u8, carry: bool) -> u8 {
        let carry = if carry { 1 } else { 0 };
        let (tmp, underflow) = self.a.overflowing_sub(val);
        let (r, underflow_c) = tmp.overflowing_sub(carry);

        self.f.z = r == 0;
        self.f.n = true;
        self.f.h = ((val & 0xf) + carry) > (self.a & 0xf);
        self.f.c = underflow || underflow_c;

        r
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
