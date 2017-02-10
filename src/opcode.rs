#[derive(Debug)]
pub struct Opcode {
    opcode: u16,
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        Opcode { opcode: opcode }
    }

    pub fn category(&self) -> u8 {
        ((self.opcode & 0xF000) >> 12) as u8
    }

    pub fn x(&self) -> usize {
        ((self.opcode & 0x0F00) >> 8) as usize
    }

    pub fn y(&self) -> usize {
        ((self.opcode & 0x00F0) >> 4) as usize
    }

    pub fn address(&self) -> u16 {
        self.opcode & 0x0FFF
    }

    pub fn get_8bit(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    pub fn get_4bit(&self) -> u8 {
        (self.opcode & 0x000F) as u8
    }
}