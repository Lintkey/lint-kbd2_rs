use crate::constants::KEY_BYTES_LEN;

pub trait KeyStates: Default {
    fn toggle(&mut self, index: usize);

    fn is_pressed(&self, index: usize) -> bool;
}

pub type BitKeyStates = [u8; KEY_BYTES_LEN];

impl KeyStates for [u8; KEY_BYTES_LEN] {
    fn toggle(&mut self, index: usize) {
        self[index/8] ^= 1<<(index%8);
    }

    fn is_pressed(&self, index: usize) -> bool {
        (self[index/8]>>(index%8))&1 == 1
    }
}