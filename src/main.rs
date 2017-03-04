#[macro_use]
extern crate clap;
extern crate sdl2;

use clap::{Arg, App};
use vm::VM;
use cartridge::Cartridge;

mod cartridge;
mod cpu;
mod interconnect;
mod mem_map;
mod memory;
mod vm;

fn main() {
    let matches = App::new("Gameboy Emulator")
                        .version(crate_version!())
                        .author("tompko  <tompko@gmail.com>")
                        .about("Emulates the Game Boy language")
                        .arg(Arg::with_name("INPUT")
                            .help("Sets the cartridge file to use")
                            .required(true)
                            .index(1))
                        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let cartridge = Cartridge::load(input_file).unwrap();

    let mut vm = VM::new(cartridge);

    vm.run();
}
