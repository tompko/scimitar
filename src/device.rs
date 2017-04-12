#[derive(Clone, Copy)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,

    Backspace,
    Enter,

    Z,
    X,
}

pub trait Device {
    fn update(&mut self);
    fn set_frame_buffer(&mut self, buffer: &[u32]);

    fn key_down(&self, key: Key) -> bool;

    fn running(&self) -> bool;
}
