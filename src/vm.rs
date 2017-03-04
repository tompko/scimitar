use super::interconnect::Interconnect;
use super::cpu::Cpu;

pub struct VM {
    inter: Interconnect,
    cpu: Cpu
}

impl VM {
    pub fn new() -> VM {
        let inter = Interconnect::new();
        let cpu = Cpu::new();
        VM{
            inter: inter,
            cpu: cpu,
        }
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.inter.load_cartridge(rom);
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.inter);
        }
    }
}
