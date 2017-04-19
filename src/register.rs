use std::ops::Index;
use std::ops::IndexMut;

pub const NUM_REGISTERS: usize = 16;

pub struct Register {
    reg_v: [u8; NUM_REGISTERS],
    pub reg_i: u16,
}

impl Register {
    pub fn new() -> Register {
        Register {
            reg_v: [0; NUM_REGISTERS],
            reg_i: 0,
        }
    }

    pub fn clear(&mut self) {
        self.reg_v = [0; NUM_REGISTERS];
        self.reg_i = 0;
    }
}

impl Index<usize> for Register {
    type Output = u8;

    fn index(&self, i: usize) -> &u8 {
        &self.reg_v[i]
    }
}

impl IndexMut<usize> for Register {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        &mut self.reg_v[i]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reg_clear() {
        let mut register = Register::new();
        register.clear();
        // assert!(register.get().iter().all(|&x| x == 0));
    }

    #[test]
    fn reg_get_set() {
        let mut register = Register::new();
        register[0] = 0xFF;
        assert_eq!(register[0], 0xFF);
    }

    #[test]
    fn reg_get_set_i() {
        let mut register = Register::new();
        register.reg_i = 1;
        assert_eq!(register.reg_i, 1);
    }
}
