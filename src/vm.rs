use interconnect::Interconnect;
use cpu::Cpu;
use device::Device;

pub struct VM<T: Interconnect> {
    cpu: Cpu,
    inter: T,
}

impl<T: Interconnect> VM<T> {
    pub fn new(interconnect: T, with_boot_rom: bool) -> VM<T> {
        let mut cpu = Cpu::new();
        if with_boot_rom {
            cpu.pc = 0x0000;
        }

        VM {
            inter: interconnect,
            cpu: cpu,
        }
    }

    pub fn step(&mut self, device: &mut Device) -> u16 {
        let cycles = self.cpu.step(&mut self.inter);

        self.inter.step(cycles, device);

        cycles
    }

    pub fn get_children(self) -> (Cpu, T) {
        (self.cpu, self.inter)
    }
}
