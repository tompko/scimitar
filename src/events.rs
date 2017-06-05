#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Event {
    Watchpoint,
    Unrecognized0xed,
}
