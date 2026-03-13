use crate::kbp::key_scanner::debounce::{DebounceKeyStates, KeyDiff};
use crate::kbp::key_scanner::key_state::KeyStates;

pub struct PingPongKeyStates<const KEY_NUM: usize, KS: KeyStates, const DEBOUNCE_THRESHOLD: u16> {
    inner: KS,
    counter: [u16; KEY_NUM],
}

impl<const KEY_NUM: usize, KS: KeyStates, const DEBOUNCE_THRESHOLD: u16> Default for PingPongKeyStates<KEY_NUM, KS, DEBOUNCE_THRESHOLD> {
    fn default() -> Self {
        Self {
            inner: KS::initial_state(),
            counter: [0; KEY_NUM],
        }
    }
}

impl<const KEY_NUM: usize, KS: KeyStates, KD: KeyDiff, const DEBOUNCE_THRESHOLD: u16> DebounceKeyStates<KS, KD> for PingPongKeyStates<KEY_NUM, KS, DEBOUNCE_THRESHOLD> {
    fn debounce(&mut self, input: &KS) -> KD {
        let mut diff = KD::default();
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
