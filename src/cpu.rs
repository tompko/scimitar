use interconnect::Interconnect;

struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

pub struct Cpu {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,

    cycle_count: u16,

    instructions_to_di: u8,
    interrupts_enabled: bool,
}

static CYCLE_COUNTS: [u16; 256] = [
    4, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0,
    0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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

            cycle_count: 0,

            instructions_to_di: 0,
            interrupts_enabled: true,
        }
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) {
        let instr = interconnect.read_byte(self.pc);
        let mut next_pc = self.pc + 1;

        match instr {
            0x00 => {
                // NOP - No Operation
            }
            0x01 => {
                // LD BC, nn
                let lsb = interconnect.read_byte(next_pc);
                let msb = interconnect.read_byte(next_pc + 1);

                next_pc += 2;
                let val = ((msb as u16) << 8) | lsb as u16;

                self.b = msb;
                self.c = lsb;
            }
            0x11 => {
                // LD DE, nn
                let lsb = interconnect.read_byte(next_pc);
                let msb = interconnect.read_byte(next_pc + 1);

                next_pc += 2;
                let val = ((msb as u16) << 8) | lsb as u16;

                self.d = msb;
                self.e = lsb;
            }
            0x21 => {
                // LD HL, nn
                let lsb = interconnect.read_byte(next_pc);
                let msb = interconnect.read_byte(next_pc + 1);

                next_pc += 2;
                let val = ((msb as u16) << 8) | lsb as u16;

                self.h = msb;
                self.l = lsb;
            }
            0x31 => {
                // LD SP, nn
                let lsb = interconnect.read_byte(next_pc);
                let msb = interconnect.read_byte(next_pc + 1);

                next_pc += 2;
                let val = ((msb as u16) << 8) | lsb as u16;

                self.sp = val;
            }
            0x3e => {
                // LD A, # - Load immediate 8-bit into A
                let val = interconnect.read_byte(next_pc);
                next_pc += 1;

                self.a = val;
            }
            0xc3 => {
                // JP nn - Jump to address nn
                let lsb = interconnect.read_byte(next_pc);
                let msb = interconnect.read_byte(next_pc + 1);

                next_pc = ((msb as u16) << 8) | lsb as u16;

                println!("JP: {} {} {}", lsb, msb, next_pc);
            }
            0xcd => {
                // CALL nn - Call function at nn
                let addr = interconnect.read_halfword(next_pc);
                next_pc += 2;

                self.push(interconnect, next_pc);
                next_pc = addr;
            }
            0xe0 => {
                // LDH (n), A - Store A in memory 0xff00+n
                let n = interconnect.read_byte(next_pc);
                next_pc += 1;

                let addr = 0xff00 + (n as u16);
                interconnect.write_byte(addr, self.a);
            }
            0xea => {
                // LD nn, A - Store A to immediate address
                let lsb = interconnect.read_byte(next_pc);
                let msb = interconnect.read_byte(next_pc + 1);
                let addr = ((msb as u16) << 8) | lsb as u16;
                next_pc += 2;

                interconnect.write_byte(addr, self.a);
            }
            0xf3 => {
                // DI -Disable interrupts after the next instruction is executed
                self.instructions_to_di = 1;
            }
            _ => panic!("Unrecognized instruction {:02x}", instr),
        }

        if CYCLE_COUNTS[instr as usize] == 0 {
            panic!("No cycle count for instruction: {:02x}", instr);
        }
        self.cycle_count += CYCLE_COUNTS[instr as usize];

        if self.instructions_to_di > 0 {
            self.instructions_to_di -= 1;
            if self.instructions_to_di == 0 {
                self.disable_interrupts();
            }
        }

        self.pc = next_pc;
    }

    fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    fn push(&mut self, interconnect: &mut Interconnect, addr: u16) {
        interconnect.write_halfword(self.sp, addr);
        self.sp -= 2;
    }
}
