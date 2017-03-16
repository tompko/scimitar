use mem_map::*;

pub struct Gpu {
    vram: Box<[u8]>, // VRAM - mapped to 0x8000 - 0x9FFF
    obj_attr_table: Box<[u8]>, // Obj/Sprite Attribute Table (OAM) - mapped to 0xfe00 - 0xfea0

    lcd_control: LcdControlReg, // 0xff40 - LCDC
    lcdc_status: LcdcStatusReg, // 0xff41 - STAT
    scy: u8, // 0xff42 - SCY scroll background Y position
    scx: u8, // 0xff43 - SCX scroll background X position
    ly: u8, // 0xff44 - LY scanline being sent to LCD driver, reset on write
    lyc: u8, // 0xff45 - LYC if equal to LY STAT.coincident is set
    dma_transfer: u8, // 0xff46 - when set initiates DMA transfer (~160 microseconds)
    bg_palette_data: PaletteDataReg, // 0xff47 - BG & Window palette data
    obj0_palette_data: PaletteDataReg, // 0xff48 OBJ0 palette data
    obj1_palette_data: PaletteDataReg, // 0xff49 OBJ1 palette data
    wy: u8, // 0xff4a - window Y position
    wx: u8, // 0xff4b - window X position, offset from screen coords by 7
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            vram: vec![0; VRAM_LENGTH as usize].into_boxed_slice(),
            obj_attr_table: vec![0; OAM_LENGTH as usize].into_boxed_slice(),

            lcd_control: LcdControlReg::default(),
            lcdc_status: LcdcStatusReg::default(),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bg_palette_data: PaletteDataReg::default(),
            obj0_palette_data: PaletteDataReg::default(),
            obj1_palette_data: PaletteDataReg::default(),
            dma_transfer: 0,
        }
    }

    // TODO - needs to read/write from oam and registers
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val;
    }
}

#[derive(Default)]
pub struct LcdControlReg {
    bg_window_display: bool,
    obj_display: bool,
    obj_size: bool,
    bg_tile_map_display:bool,
    bg_win_tile_data: bool,
    window_display: bool,
    win_tile_map_display: bool,
    lcd_control_op: bool,
}

impl LcdControlReg {
    fn default() -> Self {
        0x91.into()
    }
}

impl From<u8> for LcdControlReg {
    fn from(val: u8) -> Self {
        // TODO - implement properly
        LcdControlReg {
            bg_window_display: false,
            obj_display: false,
            obj_size: false,
            bg_tile_map_display:false,
            bg_win_tile_data: false,
            window_display: false,
            win_tile_map_display: false,
            lcd_control_op: false,
        }
    }
}


#[derive(Default)]
pub struct LcdcStatusReg {
        // TODO - implement
}

#[derive(Default)]
pub struct PaletteDataReg {
        // TODO - implement
}
