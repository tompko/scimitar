pub const ROM0_START: u16 = 0x0000;
pub const ROM0_LENGTH: u16 = 0x4000;
pub const ROM0_END: u16 = ROM0_START + ROM0_LENGTH - 1;

// pub const ROMSW_START: u16 = 0x4000;
// pub const ROMSW_LENGTH: u16 = 0x4000;
// pub const ROMSW_END: u16 = ROMSW_START + ROMSW_LENGTH - 1;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_LENGTH: u16 = 0x2000;
pub const VRAM_END: u16 = VRAM_START + VRAM_LENGTH -1;

pub const INTERNAL_RAM_START: u16 = 0xc000;
pub const INTERNAL_RAM_LENGTH: u16 = 0x2000;
pub const INTERNAL_RAM_END: u16 = INTERNAL_RAM_START + INTERNAL_RAM_LENGTH - 1;

pub const IRAM_ECHO_START: u16 = 0xe000;
pub const IRAM_ECHO_LENGTH: u16 = 0x1e00;
pub const IRAM_ECHO_END: u16 = IRAM_ECHO_START + IRAM_ECHO_LENGTH - 1;

pub const OAM_START: u16 = 0xfe00;
pub const OAM_LENGTH: u16 = 0x00a0;
pub const OAM_END: u16 = OAM_START + OAM_LENGTH - 1;

// pub const IO_PORTS_START: u16 = 0xff00;
// pub const IO_PORTS_LENGTH: u16 = 0x80;
// pub const IO_PORTS_END: u16 = IO_PORTS_START + IO_PORTS_LENGTH - 1;

pub const HIGH_RAM_START: u16 = 0xff80;
pub const HIGH_RAM_LENGTH: u16 = 0x7f;
pub const HIGH_RAM_END: u16 = HIGH_RAM_START + HIGH_RAM_LENGTH - 1;
