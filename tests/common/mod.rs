extern crate byteorder;
extern crate crc;
extern crate gameboy;

use std::path::Path;
use self::byteorder::{ByteOrder, LittleEndian};
use self::crc::crc32::checksum_ieee;
use self::gameboy::bootrom::Bootrom;
use self::gameboy::cartridge::Cartridge;
use self::gameboy::config::model::{Model, DEFAULT_MODEL_PRIORITY};
use self::gameboy::interconnect::Interconnect;
use self::gameboy::vm::VM;
use self::gameboy::device::{self, Device};
use self::gameboy::symbols::Symbols;

struct TestDevice {
    buffer: Box<[u32]>,
}

impl TestDevice {
    fn new(width: usize, height: usize) -> Self {
        TestDevice {
            buffer: vec![0; width * height].into_boxed_slice(),
        }
    }
}

impl Device for TestDevice {
    fn update(&mut self) {}

    fn set_frame_buffer(&mut self, buffer: &[u32]) {
        #[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
        for i in 0..self.buffer.len() {
            self.buffer[i] = buffer[i];
        }
    }

    fn key_down(&self, _: device::Key) -> bool {
        false
    }

    fn running(&self) -> bool {
        true
    }
}

#[allow(dead_code)]
pub fn run_test_with_hash<P: AsRef<Path>>(file_name: P, model: Model, hash: u32) {
    let cartridge = Cartridge::load(file_name.as_ref()).unwrap();
    let bootrom = Bootrom::lookup(&[model]);
    let interconnect = Interconnect::new(bootrom, cartridge);

    let mut device = TestDevice::new(interconnect.get_width(), interconnect.get_height());

    let mut vm = VM::new(interconnect, false, Symbols::default());

    for _ in 0..30000000 {
        vm.step(&mut device);
    }

    let mut bytes = Vec::new();

    for b in device.buffer.iter() {
        let mut parts = vec![0;4];
        // BigEndian::write_u32(&mut parts, *b);
        LittleEndian::write_u32(&mut parts, *b);

        for p in parts {
            bytes.push(p);
        }
    }

    assert_eq!(hash, checksum_ieee(&bytes));
}

#[allow(dead_code)]
pub fn run_test_till_ed<P: AsRef<Path>>(file_name: P, model: Model) {
    let cartridge = Cartridge::load(file_name.as_ref()).unwrap();
    let bootrom = Bootrom::lookup(&[model]);
    let interconnect = Interconnect::new(bootrom, cartridge);

    let mut device = TestDevice::new(interconnect.get_width(), interconnect.get_height());

    let mut vm = VM::new(interconnect, false, Symbols::default());

    while vm.get_next_instruction() != 0xed {
        vm.step(&mut device);
    }

    assert_eq!(vm.get_cpu().a, 0);
    assert_eq!(vm.get_cpu().b, 3);
    assert_eq!(vm.get_cpu().c, 5);
    assert_eq!(vm.get_cpu().d, 8);
    assert_eq!(vm.get_cpu().e, 13);
    assert_eq!(vm.get_cpu().h, 21);
    assert_eq!(vm.get_cpu().l, 34);
}

#[allow(dead_code)]
pub fn run_all_models_till_ed<P: AsRef<Path>>(file_name: P) {
    for m in &DEFAULT_MODEL_PRIORITY {
        run_test_till_ed(file_name.as_ref(), *m);
    }
}
