extern crate byteorder;
extern crate crc;
extern crate gameboy;

use std::path::Path;
use self::byteorder::{ByteOrder, LittleEndian};
use self::crc::crc32::checksum_ieee;
use self::gameboy::cartridge::Cartridge;
use self::gameboy::interconnect::Interconnect;
use self::gameboy::vm::VM;
use self::gameboy::device::{self, Device};

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

pub fn run_test_with_hash<P: AsRef<Path>>(file_name: P, hash: u32) {
    let cartridge = Cartridge::load(file_name).unwrap();
    let interconnect = Interconnect::new(cartridge);

    let mut device = TestDevice::new(interconnect.get_width(), interconnect.get_height());

    let mut vm = VM::new(interconnect, false, false);


    for _ in 0..25000000 {
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
