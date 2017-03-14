#[macro_use]
extern crate clap;
extern crate sdl2;
extern crate gameboy;

use clap::{Arg, App};
use gameboy::vm::VM;
use gameboy::cartridge::Cartridge;
use gameboy::interconnect::GBInterconnect;

fn main() {
    let matches = App::new("Gameboy Emulator")
        .version(crate_version!())
        .author("tompko  <tompko@gmail.com>")
        .about("Emulates the Game Boy language")
        .arg(Arg::with_name("INPUT")
                 .help("Sets the cartridge file to use")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("boot-rom")
             .help("Sets the boot rom to use")
             .short("b")
             .long("boot-rom")
             .takes_value(true))
        .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let mut cartridge = Cartridge::load(input_file).unwrap();
    let mut with_boot_rom = false;

    if let Some(boot_file) = matches.value_of("boot-rom") {
        with_boot_rom = true;
        cartridge.load_boot_rom(boot_file).unwrap();
    }
    let interconnect = GBInterconnect::new(cartridge);

    let mut vm = VM::new(interconnect, with_boot_rom);

    vm.run();
}
