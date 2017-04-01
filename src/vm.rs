use std::thread;
use interconnect::Interconnect;
use cpu::Cpu;
use device::Device;
use time::{self, SteadyTime};

// The Game Boy runs at 4194304 Hz which is 8192 clocks every 1953125 nanoseconds
const SYNC_PERIOD_NS: i64 = 1953125;
const SYNC_PERIOD_CLOCKS: i64 = 8192;

pub struct VM<T: Interconnect> {
    cpu: Cpu,
    inter: T,
}

impl<T: Interconnect> VM<T> {
    pub fn new(interconnect: T, with_boot_rom: bool) -> VM<T> {
        let mut cpu = Cpu::new();
        let mut interconnect = interconnect;
        if with_boot_rom {
            cpu.pc = 0x0000;
        } else {
            // Set the registers up as if we'd run the boot rom
            // TODO -the values don't match the reference, check once the cpu is
            // working
            cpu.a = 0x00;
            cpu.f = 0x00.into();
            cpu.set_bc(0x0000);
            cpu.set_de(0x0000);
            cpu.set_hl(0x0000);
            cpu.sp = 0xfffe;
            interconnect.write_byte(0xff05, 0x00);
            interconnect.write_byte(0xff06, 0x00);
            interconnect.write_byte(0xff07, 0x00);
            interconnect.write_byte(0xff10, 0x80);
            interconnect.write_byte(0xff11, 0xbf);
            interconnect.write_byte(0xff12, 0xf3);
            interconnect.write_byte(0xff14, 0xbf);
            interconnect.write_byte(0xff16, 0x3f);
            interconnect.write_byte(0xff17, 0x00);
            interconnect.write_byte(0xff19, 0xbf);
            interconnect.write_byte(0xff1a, 0x7f);
            interconnect.write_byte(0xff1b, 0xff);
            interconnect.write_byte(0xff1c, 0x9f);
            interconnect.write_byte(0xff1e, 0xbf);
            interconnect.write_byte(0xff20, 0xff);
            interconnect.write_byte(0xff21, 0x00);
            interconnect.write_byte(0xff22, 0x00);
            interconnect.write_byte(0xff23, 0xbf);
            interconnect.write_byte(0xff24, 0x77);
            interconnect.write_byte(0xff25, 0xf3);
            interconnect.write_byte(0xff26, 0xf1);
            interconnect.write_byte(0xff40, 0x91);
            interconnect.write_byte(0xff42, 0x00);
            interconnect.write_byte(0xff43, 0x00);
            interconnect.write_byte(0xff45, 0x00);
            interconnect.write_byte(0xff47, 0xfc);
            interconnect.write_byte(0xff48, 0xff);
            interconnect.write_byte(0xff49, 0xff);
            interconnect.write_byte(0xff4a, 0x00);
            interconnect.write_byte(0xff4b, 0x00);
            interconnect.write_byte(0xffff, 0x00);
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

    pub fn run(&mut self, device: &mut Device) {
    let mut start = SteadyTime::now();
    let mut nsecs_elapsed = 0;
    let mut cycles_to_run = 0;

    while device.running() {
        let now = SteadyTime::now();
        let elapsed = now - start;
        nsecs_elapsed += elapsed.num_nanoseconds().expect("Loop took too long");
        start = now;

        while nsecs_elapsed > SYNC_PERIOD_NS {
            cycles_to_run += SYNC_PERIOD_CLOCKS;
            while cycles_to_run > 0 {
                cycles_to_run -= self.step(device) as i64;
                device.update();
            }
            nsecs_elapsed -= SYNC_PERIOD_NS;
        }

        thread::sleep(time::Duration::milliseconds(3).to_std().unwrap());
    }
    }

    pub fn get_children(self) -> (Cpu, T) {
        (self.cpu, self.inter)
    }
}
