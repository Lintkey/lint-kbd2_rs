#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent {
    pub is_pressed: bool,
    /// 仅支持256个按键(不会有人用超过256键吧，乐)
    pub key_index: u8,
}

impl KeyEvent {
    pub fn new(is_pressed: bool, key_index: u8) -> Self {
        Self { is_pressed, key_index: key_index.into() }
    }
}