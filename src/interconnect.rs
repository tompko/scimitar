use sdl2;


pub struct Interconnect {
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let context = sdl2::init().unwrap();

        Interconnect{
        }
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
    }
}
