use crate::constants::{DEBOUNCE_THRESHOLD, KEY_NUM};
use crate::kbp::key_state::KeyStates;
use crate::kbp::key_state::BitKeyStates;

#[derive(Default)]
pub struct KeyDiffs(BitKeyStates);

pub trait KeyDiffTrait {
    fn set_different(&mut self, index: usize);
    fn is_different(&self, index: usize) -> bool;
}

impl KeyDiffTrait for KeyDiffs {
    fn set_different(&mut self, index: usize) {
        self.0[index/8] |= 1<<(index%8);
    }

    fn is_different(&self, index: usize) -> bool {
        (self.0[index/8]>>(index%8))&1 == 1
    }
}

pub trait DebounceKeyStates: Default {
    fn debounce(&mut self, input: &BitKeyStates) -> KeyDiffs;

    fn is_pressed(&self, index: usize) -> bool;
}



pub struct PingPongKeyStates {
    inner: BitKeyStates,
    counter: [u16; KEY_NUM],
}

impl Default for PingPongKeyStates {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            counter: [0; KEY_NUM],
        }
    }
}

impl DebounceKeyStates for PingPongKeyStates {
    fn debounce(&mut self, input: &BitKeyStates) -> KeyDiffs {
        let mut diff = KeyDiffs::default();
        for index in 0..KEY_NUM {
            if input.is_pressed(index) == self.inner.is_pressed(index) {
                if self.counter[index] > 0 {
                    self.counter[index] -= 1;
                }
            } else if self.counter[index] < DEBOUNCE_THRESHOLD {
                self.counter[index] += 1;
            } else {
                self.counter[index] = 0;
                self.inner.toggle(index);
                diff.set_different(index);
            }
        }
        diff
    }
    
    fn is_pressed(&self, index: usize) -> bool {
        self.inner.is_pressed(index)
    }
}