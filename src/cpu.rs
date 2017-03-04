use super::interconnect::Interconnect;


pub struct Cpu {
    pc: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            pc: 0x100,
        }
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) {
        let instr = interconnect.read_byte(self.pc);

        match instr {
            _ => panic!("Unrecognized instruction {:02x}", instr),
        }
    }
}
