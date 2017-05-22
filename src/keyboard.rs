use std::ops::{Index, IndexMut};

pub const NUM_KEYS: usize = 16;

pub struct Keyboard {
    keys: [u8; NUM_KEYS],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { keys: [0; NUM_KEYS] }
    }
}

impl Index<usize> for Keyboard {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.keys[i]
    }
}

impl IndexMut<usize> for Keyboard {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        &mut self.keys[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_set_get() {
        let mut keyboard = Keyboard::new();
        keyboard[0] = 0x1;
        assert_eq!(keyboard[0], 0x1);
    }
}

