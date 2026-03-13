use core::ops::{Index, IndexMut};

pub trait KeyStates: Default + Index<usize> + IndexMut<usize> {
    fn initial_state() -> Self;

    fn toggle(&mut self, index: usize);

    fn is_pressed(&self, index: usize) -> bool;
}

pub struct BitKeyStates<const KEY_NUM: usize> where [(); (KEY_NUM+7)/8]: {
    inner: [u8; (KEY_NUM+7)/8]
}

impl<const KEY_NUM: usize> BitKeyStates<KEY_NUM>
where [(); (KEY_NUM+7)/8]: {
    pub fn from_buffer(buffer: [u8; (KEY_NUM+7)/8]) -> Self {
        Self{ inner: buffer }
    }
}

impl<const KEY_NUM: usize> Default for BitKeyStates<KEY_NUM>
where [(); (KEY_NUM+7)/8]: {
    fn default() -> Self {
        Self{ inner: [0; _] }
    }
}

impl<const KEY_NUM: usize> Index<usize> for BitKeyStates<KEY_NUM>
where [(); (KEY_NUM+7)/8]: {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<const KEY_NUM: usize> IndexMut<usize> for BitKeyStates<KEY_NUM>
where [(); (KEY_NUM+7)/8]: {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<const KEY_NUM: usize> KeyStates for BitKeyStates<KEY_NUM>
where [(); (KEY_NUM+7)/8]: {
    fn initial_state() -> Self {
        Self{ inner: [255; _] }
    }

    fn toggle(&mut self, index: usize) {
        self[index/8] ^= 1<<(index%8);
    }

    fn is_pressed(&self, index: usize) -> bool {
        (self[index/8]>>(index%8))&1 == 0
    }
}