pub trait Device {
    fn update(&mut self);
    fn set_frame_buffer(&mut self, buffer: &[u32]);
}
