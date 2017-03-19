use mem_map::*;
use device::Device;

const COLOUR_MAP: [u32; 4] = [0xff7e8429, 0xff527a4b, 0xff315d4b, 0xff29473e];
const WIDTH: usize = 160;
const HEIGHT: usize = 144;

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

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.obj_attr_table[addr as usize]
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        self.obj_attr_table[addr as usize] = val;
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xff40 => self.lcd_control.clone().into(),
            0xff41 => self.lcdc_status.clone().into(),
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff46 => self.dma_transfer,
            0xff47 => self.bg_palette_data.clone().into(),
            0xff48 => self.obj0_palette_data.clone().into(),
            0xff49 => self.obj1_palette_data.clone().into(),
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => panic!("Read from non-gpu register in gpu {:04x}", addr),
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        match addr {
            0xff40 => self.lcd_control = val.into(),
            0xff41 => self.lcdc_status = val.into(),
            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff44 => self.ly = val,
            0xff45 => self.lyc = val,
            0xff46 => self.dma_transfer = val,
            0xff47 => self.bg_palette_data = val.into(),
            0xff48 => self.obj0_palette_data = val.into(),
            0xff49 => self.obj1_palette_data = val.into(),
            0xff4a => self.wy = val,
            0xff4b => self.wx = val,
            _ => panic!("Write to non-gpu register in gpu {:04x} = {:02x}", addr, val),
        }
    }

    pub fn step(&mut self, cycles: u16, device: &mut Device) {
        let mut buffer: Vec<u32> = vec![0xff7e8429; WIDTH * HEIGHT];

        device.set_frame_buffer(&buffer);
    }

    pub fn get_width(&self) -> usize {
        WIDTH
    }

    pub fn get_height(&self) -> usize {
        HEIGHT
    }
}

#[derive(Default, Clone)]
pub struct LcdControlReg {
    bg_window_display: bool,
    obj_display: bool,
    obj_size: bool,
    bg_tile_map_display: bool,
    bg_win_tile_data: bool,
    window_display: bool,
    win_tile_map_display: bool,
    lcd_control_op: bool,
}

impl LcdControlReg {
    fn default() -> Self {
        // bg_window_display | bg_win_tile_data | lcd_control_op
        0x91.into()
    }
}

impl From<u8> for LcdControlReg {
    fn from(val: u8) -> Self {
        LcdControlReg {
            bg_window_display: val & (1 << 0) != 0,
            obj_display: val & (1 << 1) != 0,
            obj_size: val & (1 << 2) != 0,
            bg_tile_map_display: val & (1 << 3) != 0,
            bg_win_tile_data: val & (1 << 4) != 0,
            window_display: val & (1 << 5) != 0,
            win_tile_map_display: val & (1 << 6) != 0,
            lcd_control_op: val & (1 << 7) != 0,
        }
    }
}

impl Into<u8> for LcdControlReg {
    fn into(self) -> u8 {
        let mut ret: u8 = 0;
        if self.bg_window_display {
            ret |= 1 << 0;
        }
        if self.obj_display {
            ret |= 1 << 1;
        }
        if self.obj_size {
            ret |= 1 << 2;
        }
        if self.bg_tile_map_display {
            ret |= 1 << 3;
        }
        if self.bg_win_tile_data {
            ret |= 1 << 4;
        }
        if self.window_display {
            ret |= 1 << 5;
        }
        if self.win_tile_map_display {
            ret |= 1 << 6;
        }
        if self.lcd_control_op {
            ret |= 1 << 7;
        }
        ret
    }
}


#[derive(Default, Clone)]
pub struct LcdcStatusReg {
    coincidence_interrupt_enable: bool,
    oam_interrupt_enable: bool,
    vblank_interrupt_enable: bool,
    hblank_interrupt_enable: bool,
    coincidence_flag: bool,
    mode_flag: u8,
}

impl From<u8> for LcdcStatusReg {
    fn from(val: u8) -> Self {
        LcdcStatusReg {
            coincidence_interrupt_enable: val & (1 << 6) != 0,
            oam_interrupt_enable: val & (1 << 5) != 0,
            vblank_interrupt_enable: val & (1 << 4) != 0,
            hblank_interrupt_enable: val & (1 << 3) != 0,
            coincidence_flag: val & (1 << 2) != 0,
            mode_flag: val & 0x3,
        }
    }
}

impl Into<u8> for LcdcStatusReg {
    fn into(self) -> u8 {
        let mut ret = self.mode_flag;

        if self.coincidence_interrupt_enable {
            ret |= 1 << 6;
        }
        if self.oam_interrupt_enable {
            ret |= 1 << 5;
        }
        if self.vblank_interrupt_enable {
            ret |= 1 << 4;
        }
        if self.hblank_interrupt_enable {
            ret |= 1 << 3;
        }
        if self.coincidence_flag {
            ret |= 1 << 2;
        }
        ret
    }
}

#[derive(Default, Clone)]
pub struct PaletteDataReg {
    col0_shade: u8,
    col1_shade: u8,
    col2_shade: u8,
    col3_shade: u8,
}

impl From<u8> for PaletteDataReg {
    fn from(val: u8) -> Self {
        PaletteDataReg {
            col0_shade: val & 0x3,
            col1_shade: (val >> 2) & 0x3,
            col2_shade: (val >> 4) & 0x3,
            col3_shade: (val >> 6) & 0x3,
        }
    }
}

impl Into<u8> for PaletteDataReg {
    fn into(self) -> u8 {
        self.col0_shade | (self.col1_shade << 2) | (self.col2_shade << 4) | (self.col3_shade << 6)
    }
}
