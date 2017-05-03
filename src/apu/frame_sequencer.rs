#[derive(Default)]
pub struct FrameSequencer {
    cycles: u8,
}

impl FrameSequencer {
    pub fn step(&mut self) {
        self.cycles = (self.cycles + 1) % 8;
    }

    pub fn length_step(&self) -> bool {
        self.cycles % 2 == 0
    }
}
