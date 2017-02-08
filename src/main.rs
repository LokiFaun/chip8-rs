extern crate rand;

use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

const NUM_REGISTERS: usize = 16;
const GFX_MEMORY_SIZE: usize = 64 * 32;
const MEMORY_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const PROGRAM_START: usize = 0x200;
const FONT_SET_SIZE: usize = 80;
const FONT_SET: [u8; FONT_SET_SIZE] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
     0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
     0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
     0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
     0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
     0xF0, 0x80, 0xF0, 0x80, 0x80];

struct Opcode {
    opcode: u16,
}

impl Opcode {
    fn new(opcode: u16) -> Opcode {
        Opcode { opcode: opcode }
    }

    fn category(&self) -> u8 {
        (self.opcode & 0xF000 >> 12) as u8
    }

    fn x(&self) -> usize {
        (self.opcode & 0x0F00 >> 8) as usize
    }

    fn y(&self) -> usize {
        (self.opcode & 0x00F0 >> 4) as usize
    }

    fn address(&self) -> u16 {
        self.opcode & 0x0FFF
    }

    fn get_8bit(&self) -> u8 {
        (self.opcode & 0x00FF) as u8
    }

    fn get_4bit(&self) -> u8 {
        (self.opcode & 0x000F) as u8
    }
}

struct Chip8 {
    reg_v: [u8; NUM_REGISTERS],
    reg_gfx: [u8; GFX_MEMORY_SIZE],
    memory: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    keys: [u8; NUM_KEYS],
    stack_pointer: u16,
    program_counter: u16,
    reg_i: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    fn new() -> Chip8 {
        Chip8 {
            reg_v: [0; NUM_REGISTERS],
            reg_gfx: [0; GFX_MEMORY_SIZE],
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            keys: [0; NUM_KEYS],
            stack_pointer: 0,
            program_counter: 0,
            reg_i: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn initialize(&mut self) {
        self.program_counter = PROGRAM_START as u16;
        self.reg_v = [0; NUM_REGISTERS];
        self.reg_gfx = [0; GFX_MEMORY_SIZE];
        self.stack = [0; STACK_SIZE];
        self.keys = [0; NUM_KEYS];
        self.stack_pointer = 0;
        self.reg_i = 0;
        for (index, element) in FONT_SET.into_iter().enumerate() {
            self.memory[index] = *element;
        }
    }

    fn run(&mut self) {
        loop {
            self.cycle();

            let duration = std::time::Duration::from_millis(16);
            std::thread::sleep(duration);
        }
    }

    fn load_rom(&mut self, rom: Vec<u8>) {
        for (index, element) in rom.into_iter().enumerate() {
            self.memory[index + PROGRAM_START] = element;
        }
    }

    fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        println!("OpCode={:X}", opcode.opcode);

        self.execute_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer = self.sound_timer - 1;
        }
    }

    fn fetch_opcode(&mut self) -> Opcode {
        let opcode = ((self.memory[self.program_counter as usize] as u16) << 8) +
                     (self.memory[self.program_counter as usize + 1] as u16);
        self.program_counter = self.program_counter + 2;
        Opcode::new(opcode)
    }

    fn clear_screen(&mut self) {
        // TODO
    }

    fn display(&mut self, x: usize, y: usize, height: u8) {
        // TODO
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        match opcode.category() {
            0 => {
                match opcode.get_8bit() {
                    0xE0 => self.clear_screen(),
                    0xEE => {
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.stack_pointer = self.stack_pointer - 1;
                    }
                    _ => {}
                }
            }
            1 => self.program_counter = opcode.address(),
            2 => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer = self.stack_pointer + 1;
                self.program_counter = opcode.address();
            }
            3 => {
                if self.reg_v[opcode.x()] == opcode.get_8bit() {
                    self.program_counter = self.program_counter + 2;
                }
            }
            4 => {
                if self.reg_v[opcode.x()] != opcode.get_8bit() {
                    self.program_counter = self.program_counter + 2;
                }
            }
            5 => {
                if self.reg_v[opcode.x()] == self.reg_v[opcode.y()] {
                    self.program_counter = self.program_counter + 2;
                }
            }
            6 => self.reg_v[opcode.x()] = opcode.get_8bit(),
            7 => self.reg_v[opcode.x()] = self.reg_v[opcode.x()] + opcode.get_8bit(),
            8 => {
                match opcode.get_4bit() {
                    0 => self.reg_v[opcode.x()] = self.reg_v[opcode.y()],
                    1 => self.reg_v[opcode.x()] = self.reg_v[opcode.x()] | self.reg_v[opcode.y()],
                    2 => self.reg_v[opcode.x()] = self.reg_v[opcode.x()] & self.reg_v[opcode.y()],
                    3 => self.reg_v[opcode.x()] = self.reg_v[opcode.x()] ^ self.reg_v[opcode.y()],
                    4 => {
                        self.memory[0xF] = if opcode.y() > (0xFF - opcode.x()) {
                            1
                        } else {
                            0
                        };
                        self.reg_v[opcode.x()] = self.reg_v[opcode.x()] + self.reg_v[opcode.y()]
                    }
                    5 => {
                        self.memory[0xF] = if opcode.y() > opcode.x() { 1 } else { 0 };
                        self.reg_v[opcode.x()] = self.reg_v[opcode.x()] - self.reg_v[opcode.y()]
                    }
                    6 => {
                        self.memory[0xF] = self.reg_v[opcode.x()] & 0x01;
                        self.reg_v[opcode.x()] = self.reg_v[opcode.x()] >> 1
                    }
                    7 => {
                        self.memory[0xF] = if opcode.x() > opcode.y() { 1 } else { 0 };
                        self.reg_v[opcode.x()] = self.reg_v[opcode.y()] - self.reg_v[opcode.x()]
                    }
                    0xE => {
                        self.memory[0xF] = self.reg_v[opcode.x()] & 0x80;
                        self.reg_v[opcode.x()] = self.reg_v[opcode.x()] << 1
                    }
                    _ => {}
                }
            }
            9 => {
                if self.reg_v[opcode.x()] != self.reg_v[opcode.y()] {
                    self.program_counter = self.program_counter + 2;
                }
            }
            0xA => self.reg_i = opcode.address(),
            0xB => self.program_counter = opcode.address() + self.reg_v[opcode.x()] as u16,
            0xC => self.reg_v[opcode.x()] = rand::random::<u8>() & opcode.get_8bit(),
            0xD => self.display(opcode.x(), opcode.y(), opcode.get_4bit()),
            0xE => {
                match opcode.get_8bit() {
                    0x9E => {
                        if self.keys[self.reg_v[opcode.x()] as usize] != 0 {
                            self.program_counter = self.program_counter + 2;
                        }
                    }
                    0xA1 => {
                        if self.keys[self.reg_v[opcode.x()] as usize] == 0 {
                            self.program_counter = self.program_counter + 2;
                        }
                    }
                    _ => {}
                }
            }
            0xF => {
                match opcode.get_8bit() {
                    0x07 => self.reg_v[opcode.x()] = self.delay_timer,
                    0x0A => {
                        // TODO
                    }
                    0x15 => self.delay_timer = self.reg_v[opcode.x()],
                    0x18 => self.sound_timer = self.reg_v[opcode.x()],
                    0x1E => {
                        // TODO
                    }
                    0x29 => {
                        // TODO
                    }
                    0x33 => {
                        // TODO
                    }
                    0x55 => {
                        // TODO
                    }
                    0x65 => {
                        // TODO
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() {
    if let Some(rom_file_name) = env::args().nth(1) {
        println!("Reading ROM: {}...", rom_file_name);
        let rom = read_rom(rom_file_name).unwrap_or_else(|err| {
            println!("Could not read ROM: {}!", err);
            Vec::new()
        });

        if !rom.is_empty() {
            println!("Starting CHIP-8 emulator...");
            let mut chip = Chip8::new();
            chip.initialize();
            chip.load_rom(rom);
            chip.run();
        }
    } else {
        println!("Usage: chip8 <rom>");
    }
}

fn read_rom<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = try!(fs::File::open(path));
    let mut file_buffer = Vec::new();
    try!(file.read_to_end(&mut file_buffer));
    Ok(file_buffer)
}
