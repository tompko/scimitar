use mem_map::*;
use device::Device;
use interrupt::{Irq, Interrupt};

#[allow(dead_code)] // TODO - remove when the rendering is written
const COLOUR_MAP: [u32; 4] = [0xff7e8429, 0xff527a4b, 0xff315d4b, 0xff29473e];
const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct Gpu {
    vram: Box<[u8]>, // VRAM - mapped to 0x8000 - 0x9FFF
    oam: Box<[u8]>, // Obj/Sprite Attribute Table - mapped to 0xfe00 - 0xfea0
    frame_buffer: Box<[u32]>,

    lcd_control: LcdControlReg, // 0xff40 - LCDC
    lcdc_status: LcdcStatusReg, // 0xff41 - STAT
    scy: u8, // 0xff42 - SCY scroll background Y position
    scx: u8, // 0xff43 - SCX scroll background X position
    ly: u8, // 0xff44 - LY scanline being sent to LCD driver, reset on write
    lyc: u8, // 0xff45 - LYC if equal to LY STAT.coincident is set
    bg_palette_data: PaletteDataReg, // 0xff47 - BG & Window palette data
    obj0_palette_data: PaletteDataReg, // 0xff48 OBJ0 palette data
    obj1_palette_data: PaletteDataReg, // 0xff49 OBJ1 palette data
    wy: u8, // 0xff4a - window Y position
    wx: u8, // 0xff4b - window X position, offset from screen coords by 7

    cycles: u16,
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            vram: vec![0; VRAM_LENGTH as usize].into_boxed_slice(),
            oam: vec![0; OAM_LENGTH as usize].into_boxed_slice(),
            frame_buffer: vec![COLOUR_MAP[0]; WIDTH * HEIGHT].into_boxed_slice(),

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

            cycles: 0,
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[addr as usize]
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        self.oam[addr as usize] = val;
    }

    pub fn read_reg(&self, addr: u16) -> u8 {
        match addr {
            0xff40 => self.lcd_control.into(),
            0xff41 => self.lcdc_status.into(),
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff47 => self.bg_palette_data.into(),
            0xff48 => self.obj0_palette_data.into(),
            0xff49 => self.obj1_palette_data.into(),
            0xff4a => self.wy,
            0xff4b => self.wx,
            _ => 0xff, // reads from unused addresses return 0xff
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
            0xff47 => self.bg_palette_data = val.into(),
            0xff48 => self.obj0_palette_data = val.into(),
            0xff49 => self.obj1_palette_data = val.into(),
            0xff4a => self.wy = val,
            0xff4b => self.wx = val,
            _ => {}
        }
    }

    pub fn step(&mut self, cycles: u16, device: &mut Device, irq: &mut Irq) {
        if self.lcd_control.lcd_control_op {
            for _ in 0..cycles {
                self.inner_step(device, irq);
            }
        }
    }

    pub fn get_width(&self) -> usize {
        WIDTH
    }

    pub fn get_height(&self) -> usize {
        HEIGHT
    }

    fn inner_step(&mut self, device: &mut Device, irq: &mut Irq) {
        self.cycles += 1;

        if self.lcdc_status.mode == 2 && self.cycles == 85 {
            // End of OAM search
            self.lcdc_status.mode = 3;
        } else if self.lcdc_status.mode == 3 && self.cycles == 256 {
            // This interrupt occurs on the cycle before we switch to hblank (mode 0)
            if self.lcdc_status.hblank_interrupt_enable {
                irq.raise_interrupt(Interrupt::Stat);
            }
        } else if self.lcdc_status.mode == 3 && self.cycles == 260 {
            // TODO - this mode should have variable length
            self.render_background();

            if self.lcd_control.sprite_display {
                self.render_sprites();
            }

            self.lcdc_status.mode = 0
        } else if self.lcdc_status.mode == 0 && self.cycles == 456 {
            self.ly += 1;
            self.cycles = 0;

            self.lcdc_status.coincidence_flag = self.ly == self.lyc;
            if self.lcdc_status.coincidence_interrupt_enable && self.ly == self.lyc {
                irq.raise_interrupt(Interrupt::Stat);
            }

            if self.ly < 144 {
                self.lcdc_status.mode = 2;
            } else {
                self.lcdc_status.mode = 1;

                irq.raise_interrupt(Interrupt::VBlank);
                if self.lcdc_status.vblank_interrupt_enable {
                    irq.raise_interrupt(Interrupt::Stat);
                }

                device.set_frame_buffer(&self.frame_buffer);
            }
        } else if self.lcdc_status.mode == 1 {
            if self.cycles == 456 {
                self.cycles = 0;

                if self.ly == 153 {
                    self.lcdc_status.mode = 2;
                    if self.lcdc_status.oam_interrupt_enable {
                        irq.raise_interrupt(Interrupt::Stat);
                    }
                    self.ly = 0;
                } else {
                    self.ly += 1;
                }
            }
        }
    }

    fn render_background(&mut self) {
        let background_row = self.ly.wrapping_add(self.scy);

        for i in 0..WIDTH as u8 {
            let background_col = self.scx.wrapping_add(i);

            let tile_offset = self.get_background_tile_offset(background_row / 8,
                                                              background_col / 8);
            let colour = self.get_tile_pixel(tile_offset, background_row % 8, background_col % 8);

            self.frame_buffer[(self.ly as usize * WIDTH) + i as usize] = colour;
        }
    }

    fn render_sprites(&mut self) {
        let sprite_height: i16 = if self.lcd_control.sprite_size { 16 } else { 8 };
        let ly = self.ly as i16;

        for i in 0..40 {
            let sprite_y = (self.oam[i * 4] as i16) - 16;

            if ly < sprite_y || (sprite_y + sprite_height - 1) < ly {
                continue
            }

            let sprite_row = (ly - sprite_y) as usize;
            let sprite_x = (self.oam[(i * 4) + 1] as i16) - 8;
            let tile_index = self.oam[(i * 4) + 2] as usize;
            let tile_offset = tile_index * (sprite_height as usize) * 2;

            let sprite_flags = self.oam[(i * 4) + 3];

            let flip_horz = (sprite_flags & (1 << 5)) != 0;

            for i in 0..8 {
                let x = if flip_horz { 7 - i } else { i };
                let colour = self.get_sprite_pixel(tile_offset, sprite_row, x);

                self.frame_buffer[(ly as usize * WIDTH) + (sprite_x + i as i16) as usize] = colour;
            }
        }
    }

    // Returns the offset in self.vram of the background tile
    fn get_background_tile_offset(&self, row: u8, col: u8) -> usize {
        let tile_idx_base = if self.lcd_control.bg_tile_map_display {
            0x1c00
        } else {
            0x1800
        };
        let tile_idx_offset = (row as usize * 32) + col as usize;
        let tile_index = self.vram[tile_idx_base + tile_idx_offset];

        if self.lcd_control.bg_win_tile_data {
            tile_index as usize * 16
        } else {
            (((tile_index as i8) as isize) + 256) as usize * 16
        }
    }

    fn get_tile_pixel(&self, tile_offset: usize, row: u8, col: u8) -> u32 {
        let offset = tile_offset + (row as usize * 2);

        let upper_col = self.vram[offset + 1] >> (7 - col) & 1;
        let lower_col = self.vram[offset] >> (7 - col) & 1;
        let tile_colour = upper_col << 1 | lower_col;

        let col_index = match tile_colour {
            0 => self.bg_palette_data.col0_shade,
            1 => self.bg_palette_data.col1_shade,
            2 => self.bg_palette_data.col2_shade,
            3 => self.bg_palette_data.col3_shade,
            _ => unreachable!(),
        };
        COLOUR_MAP[col_index]
    }

    fn get_sprite_pixel(&self, tile_offset: usize, row: usize, col: usize) -> u32 {
        let offset = tile_offset + (row as usize * 2);

        let upper_col = self.vram[offset + 1] >> (7 - col) & 1;
        let lower_col = self.vram[offset] >> (7 - col) & 1;
        let tile_colour = upper_col << 1 | lower_col;

        let col_index = match tile_colour {
            0 => self.obj0_palette_data.col0_shade,
            1 => self.obj0_palette_data.col1_shade,
            2 => self.obj0_palette_data.col2_shade,
            3 => self.obj0_palette_data.col3_shade,
            _ => unreachable!(),
        };
        COLOUR_MAP[col_index]
    }
}

#[derive(Default, Copy, Clone)]
pub struct LcdControlReg {
    bg_window_display: bool,
    sprite_display: bool,
    sprite_size: bool,
    bg_tile_map_display: bool,
    bg_win_tile_data: bool,
    window_display: bool,
    win_tile_map_display: bool,
    lcd_control_op: bool,
}

impl From<u8> for LcdControlReg {
    fn from(val: u8) -> Self {
        LcdControlReg {
            bg_window_display: val & 1 != 0,
            sprite_display: val & (1 << 1) != 0,
            sprite_size: val & (1 << 2) != 0,
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
            ret |= 1;
        }
        if self.sprite_display {
            ret |= 1 << 1;
        }
        if self.sprite_size {
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


#[derive(Copy, Clone)]
pub struct LcdcStatusReg {
    coincidence_interrupt_enable: bool,
    oam_interrupt_enable: bool,
    vblank_interrupt_enable: bool,
    hblank_interrupt_enable: bool,
    coincidence_flag: bool,
    mode: u8,
}

impl Default for LcdcStatusReg {
    fn default() -> Self {
        LcdcStatusReg {
            coincidence_interrupt_enable: false,
            oam_interrupt_enable: false,
            vblank_interrupt_enable: false,
            hblank_interrupt_enable: false,
            coincidence_flag: false,
            mode: 2, // Start in OAM access mode
        }
    }
}

impl From<u8> for LcdcStatusReg {
    fn from(val: u8) -> Self {
        LcdcStatusReg {
            coincidence_interrupt_enable: val & (1 << 6) != 0,
            oam_interrupt_enable: val & (1 << 5) != 0,
            vblank_interrupt_enable: val & (1 << 4) != 0,
            hblank_interrupt_enable: val & (1 << 3) != 0,
            coincidence_flag: val & (1 << 2) != 0,
            mode: val & 0x3,
        }
    }
}

impl Into<u8> for LcdcStatusReg {
    fn into(self) -> u8 {
        let mut ret = self.mode;

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

#[derive(Default, Copy, Clone)]
pub struct PaletteDataReg {
    col0_shade: usize,
    col1_shade: usize,
    col2_shade: usize,
    col3_shade: usize,
}

impl From<u8> for PaletteDataReg {
    fn from(val: u8) -> Self {
        let val = val as usize;
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
        (self.col0_shade | (self.col1_shade << 2) | (self.col2_shade << 4) |
         (self.col3_shade << 6)) as u8
    }
}
