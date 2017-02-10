#[derive(Debug)]
pub struct Opcode {
    opcode: u16,
    pub x: usize,
    pub y: usize,
    pub address: u16,
    pub category: u8,
    pub byte: u8,
    pub nibble: u8,
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        Opcode {
            opcode: opcode,
            x: ((opcode & 0x0F00) >> 8) as usize,
            y: ((opcode & 0x00F0) >> 4) as usize,
            address: opcode & 0x0FFF,
            category: ((opcode & 0xF000) >> 12) as u8,
            nibble: (opcode & 0x000F) as u8,
            byte: (opcode & 0x00FF) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_category() {
        assert_eq!(Opcode::new(0x1234).category, 1);
    }

    #[test]
    fn get_x() {
        assert_eq!(Opcode::new(0x1234).x, 2);
    }

    #[test]
    fn get_y() {
        assert_eq!(Opcode::new(0x1234).y, 3);
    }

    #[test]
    fn get_address() {
        assert_eq!(Opcode::new(0x1234).address, 0x234);
    }

    #[test]
    fn get_byte() {
        assert_eq!(Opcode::new(0x1234).byte, 0x34);
    }

    #[test]
    fn get_nibble() {
        assert_eq!(Opcode::new(0x1234).nibble, 4);
    }
}