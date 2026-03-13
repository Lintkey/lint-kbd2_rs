use super::key_state::{KeyStates, BitKeyStates};

pub trait KeyDiff: Default {
    fn set_different(&mut self, index: usize);
    fn is_different(&self, index: usize) -> bool;
}

pub trait DebounceKeyStates<KS: KeyStates, KD: KeyDiff> {
    fn debounce(&mut self, input: &KS) -> KD;

    fn is_pressed(&self, index: usize) -> bool;
}

impl<const KEY_NUM: usize> KeyDiff for BitKeyStates<KEY_NUM> where [(); (KEY_NUM+7)/8]: {
    fn set_different(&mut self, index: usize) {
        self[index/8] |= 1<<(index%8);
    }

    fn is_different(&self, index: usize) -> bool {
        (self[index/8]>>(index%8))&1 == 1
    }
}