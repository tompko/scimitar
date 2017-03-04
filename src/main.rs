#[macro_use]
extern crate clap;
extern crate sdl2;

use std::fs::File;
use std::io::Read;
use clap::{Arg, App};
use vm::VM;

mod cpu;
mod interconnect;
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
    let rom = read_rom(input_file);

    let mut vm = VM::new();

    vm.load_cartridge(rom);
    vm.run();
}

fn read_rom(filename: &str) -> Vec<u8> {
    let mut buffer = Vec::new();

    match File::open(filename) {
        Ok(ref mut file) => {
            file.read_to_end(&mut buffer).unwrap();
        },
        Err(err) => {
            println!("gameboy: cannot open '{}': {}", filename, err);
            std::process::exit(-1);
        }
    }


    buffer
}
