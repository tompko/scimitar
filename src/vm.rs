use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::io::{stdin, stdout, Write};
use std::collections::HashSet;
use interconnect::Interconnect;
use cpu::Cpu;
use device::Device;
use time::{self, SteadyTime};
use command::*;
use opcodes::*;

// The Game Boy runs at 4194304 Hz which is 8192 clocks every 1953125 nanoseconds
const SYNC_PERIOD_NS: i64 = 1953125;
const SYNC_PERIOD_CLOCKS: i64 = 8192;

#[derive(PartialEq, Eq, Debug)]
enum Mode {
    Running,
    Debugging,
}

pub struct VM {
    cpu: Cpu,
    inter: Interconnect,

    mode: Mode,
    start_time: SteadyTime,

    breakpoints: HashSet<u16>,
    cursor: u16,
    last_command: Option<Command>,
    stdin_receiver: Receiver<String>,
}

impl VM {
    pub fn new(interconnect: Interconnect, with_boot_rom: bool) -> VM {
        let (stdin_sender, stdin_receiver) = channel();

        // Blocking stdin means it's impossible to join this thread, so we let
        // the OS clean it up when we quit.
        thread::spawn(move || loop {
                          stdin_sender.send(read_stdin()).unwrap();
                      });

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

        let cursor = cpu.pc;

        let vm = VM {
            inter: interconnect,
            cpu: cpu,

            mode: Mode::Running,
            start_time: SteadyTime::now(),

            breakpoints: HashSet::new(),
            cursor: cursor,
            last_command: None,
            stdin_receiver: stdin_receiver,
        };
        if vm.mode == Mode::Debugging {
            vm.disassemble_instruction();
            vm.print_cursor();
        }
        vm
    }

    pub fn step(&mut self, device: &mut Device) -> (u16, bool) {
        let cycles = self.cpu.step(&mut self.inter);

        let start_debugger = self.inter.step(cycles, device);
        let breakpoint = self.breakpoints.contains(&self.cpu.pc);


        (cycles, start_debugger || breakpoint)
    }

    pub fn run(&mut self, device: &mut Device) {
        let mut nsecs_elapsed = 0;
        let mut cycles_to_run = 0;

        while device.running() {
            match self.mode {
                Mode::Running => {
                    let now = SteadyTime::now();
                    let elapsed = now - self.start_time;
                    nsecs_elapsed += elapsed.num_nanoseconds().expect("Loop took too long");
                    self.start_time = now;

                    while device.running() && nsecs_elapsed > SYNC_PERIOD_NS {
                        cycles_to_run += SYNC_PERIOD_CLOCKS;
                        while device.running() && cycles_to_run > 0 {
                            let (cycles_run, start_debugger) = self.step(device);
                            if start_debugger {
                                self.mode = Mode::Debugging;
                                cycles_to_run = 0;
                                self.cursor = self.cpu.pc;
                                self.print_cursor();
                                nsecs_elapsed = 0;
                                break;
                            }
                            cycles_to_run -= cycles_run as i64;
                            device.update();
                        }
                        nsecs_elapsed -= SYNC_PERIOD_NS;
                    }
                }
                Mode::Debugging => {
                    if self.run_debug_commands(device) {
                        break;
                    }

                    device.update();
                }
            }

            thread::sleep(time::Duration::milliseconds(3).to_std().unwrap());
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    fn run_debug_commands(&mut self, device: &mut Device) -> bool {
        while let Ok(command_string) = self.stdin_receiver.try_recv() {
            let command = match (command_string.parse(), self.last_command.clone()) {
                (Ok(Command::Repeat), Some(c)) => Ok(c),
                (Ok(Command::Repeat), None) => Err("No last command".into()),
                (Ok(c), _) => Ok(c),
                (Err(e), _) => Err(e),
            };

            match command {
                Ok(Command::ShowRegs) => {
                    println!("PC: {:04x}", self.cpu.pc);
                    println!("AF: {:04x}", self.cpu.af());
                    println!("BC: {:04x}", self.cpu.bc());
                    println!("DE: {:04x}", self.cpu.de());
                    println!("HL: {:04x}", self.cpu.hl());
                    println!("SP: {:04x}", self.cpu.sp);
                }
                Ok(Command::Step(count)) => {
                    for _ in 0..count {
                        self.step(device);
                        self.cursor = self.cpu.pc;
                        self.disassemble_instruction();
                    }
                }
                Ok(Command::Continue) => {
                    self.mode = Mode::Running;
                    self.start_time = SteadyTime::now();
                }
                Ok(Command::Goto(addr)) => {
                    self.cursor = addr;
                }
                Ok(Command::ShowMem(addr)) => {
                    if let Some(addr) = addr {
                        self.cursor = addr;
                    }

                    const NUM_ROWS: usize = 16;
                    const NUM_COLS: usize = 16;
                    for _ in 0..NUM_ROWS {
                        print!("0x{:08x}  ", self.cursor);
                        for x in 0..NUM_COLS {
                            let byte = self.inter.read_byte(self.cursor);
                            self.cursor = self.cursor.wrapping_add(1);
                            print!("{:02x}", byte);
                            if x < NUM_COLS - 1 {
                                print!(" ");
                            }
                        }
                        println!();
                    }
                }
                Ok(Command::Disassemble(count)) => {
                    let old_cursor = self.cursor;
                    for _ in 0..count {
                        self.cursor = self.disassemble_instruction();
                    }
                    self.cursor = old_cursor;
                }
                Ok(Command::Breakpoint) => {
                    for addr in &self.breakpoints {
                        println!("* 0x{:08x}", addr);
                    }
                }
                Ok(Command::AddBreakpoint(addr)) => {
                    self.breakpoints.insert(addr);
                }
                Ok(Command::RemoveBreakpoint(addr)) => {
                    if !self.breakpoints.remove(&addr) {
                        println!("Breakpoint at 0x{:08x} does not exist", addr);
                    }
                }
                Ok(Command::Watchpoint) => {
                    for addr in &self.inter.watchpoints {
                        println!("* 0x{:08x}", addr);
                    }
                }
                Ok(Command::AddWatchpoint(addr)) => {
                    self.inter.watchpoints.insert(addr);
                }
                Ok(Command::RemoveWatchpoint(addr)) => {
                    if !self.inter.watchpoints.remove(&addr) {
                        println!("Watchpoint at 0x{:08x} does not exist", addr);
                    }
                }
                Ok(Command::Exit) => {
                    return true;
                }
                Ok(Command::Repeat) => unreachable!(),
                Err(ref e) => println!("{}", e),
            }

            if let Ok(c) = command {
                self.last_command = Some(c);
            }

            if self.mode == Mode::Debugging {
                self.print_cursor();
            }
        }

        false
    }

    fn print_cursor(&self) {
        print!("gb-rs 0x{:04x} >>> ", self.cursor);
        stdout().flush().unwrap();
    }

    fn disassemble_instruction(&self) -> u16 {
        if self.breakpoints.contains(&self.cursor) {
            print!("* ");
        } else {
            print!("  ");
        }

        print!("0x{:08x}  ", self.cursor);
        let opcode = decode_instr(&self.inter, self.cursor);

        println!("{}", opcode);

        self.cursor + opcode.opcode_length
    }
}


fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
