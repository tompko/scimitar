use std::io::{self, Read};
use std::fmt;
use std::fs::File;
use std::path::Path;

const ROM_TYPE_OFFSET: usize = 0x0147;

enum Mbc {
    NONE,
    MBC1,
}

impl From<u8> for Mbc {
    fn from(val: u8) -> Self {
        match val {
            0x00 | 0x08 | 0x09 => Mbc::NONE,
            0x01 | 0x02 | 0x03 => Mbc::MBC1,
            _ => panic!("Unknown cartridge type {:02x}", val),
        }
    }
}

impl fmt::Display for Mbc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   Mbc::NONE => "None",
                   Mbc::MBC1 => "MBC1",
               })
    }
}

pub struct Cartridge {
    rom: Box<[u8]>,
    ram: Box<[u8]>,
    boot_rom: Box<[u8]>,

    boot_rom_active: bool,

    mbc: Mbc,
    rom_bank_lower: usize,
    bank_upper: usize,
    ram_banking: bool,

    rom_offsets: (usize, usize),
    ram_bank_offset: usize,
    ram_enabled: bool,
}

impl Cartridge {
    pub fn load<P: AsRef<Path>>(file_name: P) -> io::Result<Cartridge> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let cart = Cartridge::from_bytes(&buffer);

        println!("Loaded {:0x} bytes of cart", buffer.len());
        println!("{}", cart.name());
        println!("Cart type: {}", cart.type_name());
        println!("MBC: {}", cart.mbc);
        println!("Rom Size: {}", cart.rom_size());

        Ok(cart)
    }

    pub fn load_boot_rom<P: AsRef<Path>>(&mut self, file_name: P) -> io::Result<()> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        self.boot_rom = buffer.into_boxed_slice();
        self.boot_rom_active = true;

        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Cartridge {
        let bytes_copy = bytes.to_vec();

        let rom_type = bytes[ROM_TYPE_OFFSET];
        let mbc: Mbc = rom_type.into();

        Cartridge {
            rom: bytes_copy.into_boxed_slice(),
            ram: vec![0; 0x10000].into_boxed_slice(),
            boot_rom: Box::default(),

            boot_rom_active: false,
            mbc: mbc,

            rom_bank_lower: 1,
            bank_upper: 0,
            ram_banking: false,

            rom_offsets: (0x0000, 0x4000),
            ram_bank_offset: 0,
            ram_enabled: false,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        let (lower, upper) = self.rom_offsets;

        match addr {
            0...0x00ff if self.boot_rom_active => self.boot_rom[addr],
            0...0x3fff => self.rom[lower + addr],
            0x4000...0x7fff => self.rom[upper + (addr - 0x4000)],
            0xa000...0xbfff => {
                self.ram[self.ram_bank_offset + (addr - 0xa000)]
            }
            _ => panic!("Unrecognized read address in cartridge {:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            0x0000...0x1fff => self.ram_enabled = (val & 0x0a) != 0,
            0x2000...0x3fff => {
                let val = (val as usize) & 0x1f;
                self.rom_bank_lower = if val == 0x00 { 0x01 } else { val };
                self.update_rom_offset();
            }
            0x4000...0x5fff => {
                self.bank_upper = (val as usize) & 0x3;
                self.update_rom_offset();
                self.update_ram_offset();
            }
            0x6000...0x7fff => {
                self.ram_banking = val != 0;
                self.update_rom_offset();
                self.update_ram_offset();
            }
            0xa000...0xbfff => {
                self.ram[self.ram_bank_offset + (addr - 0xa000)] = val;
            }
            _ => {
                panic!("Unrecognized write address in cartridge {:04x}={:02x}",
                       addr,
                       val)
            }
        }
    }

    pub fn disable_boot_rom(&mut self) {
        self.boot_rom_active = false;
    }

    fn update_rom_offset(&mut self) {
        let bank_upper = self.bank_upper << 5;
        let lower = if self.ram_banking { bank_upper } else { 0x00 };
        let upper = bank_upper | self.rom_bank_lower;

        let lower_bank = (lower * 0x4000) & (self.rom.len() - 1);
        let upper_bank = (upper * 0x4000) & (self.rom.len() - 1);

        self.rom_offsets = (lower_bank, upper_bank);
    }

    fn update_ram_offset(&mut self) {
        self.ram_bank_offset = if self.ram_banking {
            self.bank_upper * 0x2000
        } else {
            0
        };
    }

    fn name(&self) -> String {
        let mut ret = "".to_string();

        for i in 0..16 {
            let c = self.rom[0x0134 + i];
            if c == 0 {
                break;
            }
            ret.push(c as char);
        }

        ret
    }

    fn rom_type(&self) -> u8 {
        self.rom[0x0147]
    }

    fn type_name(&self) -> &'static str {
        match self.rom_type() {
            0x00 => "ROM ONLY",
            0x01 => "ROM+MBC1",
            0x02 => "ROM+MBC1+RAM",
            0x03 => "ROM+MBC1+RAM+BATT",
            0x05 => "ROM+MBC2",
            0x06 => "ROM+MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0B => "ROM+MMM01",
            0x0C => "ROM+MMM01+SRAM",
            0x0D => "ROM+MMM01+SRAM+BATT",
            0x0F => "ROM+MBC3+TIMER+BATT",
            0x10 => "ROM+MBC3+TIMER+RAM+BATT",
            0x11 => "ROM+MBC3",
            0x12 => "ROM+MBC3+RAM",
            0x13 => "ROM+MBC3+RAM+BATT",
            0x15 => "MBC4",
            0x16 => "MBC4+RAM",
            0x17 => "MBC4+RAM+BATTERY",
            0x19 => "ROM+MBC5",
            0x1A => "ROM+MBC5+RAM",
            0x1B => "ROM+MBC5+RAM+BATT",
            0x1C => "ROM+MBC5+RUMBLE",
            0x1D => "ROM+MBC5+RUMBLE+SRAM",
            0x1E => "ROM+MBC5+RUMBLE+SRAM+BATT",
            0x1F => "Pocket Camera",
            0x20 => "MBC6",
            0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC => "POCKET CAMERA",
            0xFD => "Bandai TAMA5",
            0xFE => "Hudson HuC-3",
            0xFF => "Hudson HuC-1",
            _ => "UNRECOGNISED CART TYPE",
        }
    }

    fn rom_size(&self) -> &'static str {
        match self.rom[0x0148] {
            0x00 => "32KByte (no ROM banking)",
            0x01 => "64KByte (4 banks)",
            0x02 => "128KByte (8 banks)",
            0x03 => "256KByte (16 banks)",
            0x04 => "512KByte (32 banks)",
            0x05 => "1MByte (64 banks)  - only 63 banks used by MBC1",
            0x06 => "2MByte (128 banks) - only 125 banks used by MBC1",
            0x07 => "4MByte (256 banks)",
            0x08 => "8MByte (512 banks)",
            0x52 => "1.1MByte (72 banks)",
            0x53 => "1.2MByte (80 banks)",
            0x54 => "1.5MByte (96 banks)",
            _ => "UNRECOGNISED CART ROM SIZE",
        }
    }
}
