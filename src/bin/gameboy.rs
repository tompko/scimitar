#[macro_use]
extern crate clap;
extern crate gameboy;
extern crate minifb;

use clap::{Arg, App};
use minifb::{Key, Scale, WindowOptions, Window};
use gameboy::vm::VM;
use gameboy::cartridge::Cartridge;
use gameboy::interconnect::{GBInterconnect, Interconnect};
use gameboy::device::Device;

struct ConsoleDevice {
    buffer: Box<[u32]>,
    window: Window,

    width: usize,
    height: usize,
}

impl ConsoleDevice {
    fn new(window: Window, width: usize, height: usize) -> Self {
        ConsoleDevice {
            buffer: vec![0; width * height].into_boxed_slice(),
            window: window,
            width: width,
            height: height,
        }

    }
}

impl Device for ConsoleDevice {
    fn update(&mut self) {
        self.window.update_with_buffer(&*self.buffer);
    }

    fn set_frame_buffer(&mut self, buffer: &[u32]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width) + x;
                self.buffer[index as usize] = buffer[index as usize];
            }
        }
    }
}

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
    let width = interconnect.get_width();
    let height = interconnect.get_height();

    let mut vm = VM::new(interconnect, with_boot_rom);

    let window_options = WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
    };

    let window = Window::new("GBrs", width, height, window_options).unwrap();

    let mut device = ConsoleDevice::new(window, width, height);

    while device.window.is_open() && !device.window.is_key_down(Key::Escape) {
        vm.step(&mut device);
        device.update();
    }
}
