#[derive(Debug)]
pub struct Opcode {
    opcode: u16,
    pub x: usize,
    pub y: usize,
    pub address: u16,
    pub category: u8,
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        Opcode {
            opcode: opcode,
            x: ((opcode & 0x0F00) >> 8) as usize,
            y: ((opcode & 0x00F0) >> 4) as usize,
            address: opcode & 0x0FFF,
            category: ((opcode & 0xF000) >> 12) as u8,
        }
    }

    pub fn get_8bit(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    pub fn get_4bit(&self) -> u8 {
        (self.opcode & 0x000F) as u8
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
    fn get_8bit() {
        assert_eq!(Opcode::new(0x1234).get_8bit(), 0x34);
    }

    #[test]
    fn get_4bit() {
        assert_eq!(Opcode::new(0x1234).get_4bit(), 4);
    }
}