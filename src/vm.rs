use super::interconnect::Interconnect;
use super::cpu::Cpu;
use cartridge::Cartridge;

pub struct VM {
    inter: Interconnect,
    cpu: Cpu,
}

impl VM {
    pub fn new(cartridge: Cartridge) -> VM {
        let inter = Interconnect::new(cartridge);
        let cpu = Cpu::new();
        VM{
            inter: inter,
            cpu: cpu,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.step(&mut self.inter);
        }
    }
}
