use super::*;

#[cfg(not(test))]
use std::time;
#[cfg(not(test))]
use std::thread;

#[cfg(not(test))]
use sdl2::pixels;
#[cfg(not(test))]
use sdl2::event::Event;
#[cfg(not(test))]
use sdl2::keyboard::Keycode;

use opcode::Opcode;

#[cfg(not(test))]
const PIXEL_SIZE: usize = 20;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;
const GFX_MEMORY_SIZE: usize = DISPLAY_HEIGHT * DISPLAY_WIDTH;
const NUM_REGISTERS: usize = 16;
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

pub struct Chip8 {
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
    refresh: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
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
            refresh: true,
        }
    }

    pub fn initialize(&mut self) {
        self.program_counter = PROGRAM_START as u16;
        self.reg_v = [0; NUM_REGISTERS];
        self.memory = [0; MEMORY_SIZE];
        self.reg_gfx = [0; GFX_MEMORY_SIZE];
        self.stack = [0; STACK_SIZE];
        self.keys = [0; NUM_KEYS];
        self.stack_pointer = 0;
        self.reg_i = 0;
        self.refresh = true;
        for (index, element) in FONT_SET.into_iter().enumerate() {
            self.memory[index] = *element;
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (index, element) in rom.into_iter().enumerate() {
            self.memory[index + PROGRAM_START] = element;
        }
    }

    #[cfg(not(test))]
    pub fn run(&mut self) {
        let sdl_context = super::sdl2::init().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys.window("chip8",
                    (DISPLAY_WIDTH * PIXEL_SIZE) as u32,
                    (DISPLAY_HEIGHT * PIXEL_SIZE) as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        let mut events = sdl_context.event_pump().unwrap();

        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            Keycode::Num0 => self.keys[0] = 1,
                            Keycode::Num1 => self.keys[1] = 1,
                            Keycode::Num2 => self.keys[2] = 1,
                            Keycode::Num3 => self.keys[3] = 1,
                            Keycode::Num4 => self.keys[4] = 1,
                            Keycode::Num5 => self.keys[5] = 1,
                            Keycode::Num6 => self.keys[6] = 1,
                            Keycode::Num7 => self.keys[7] = 1,
                            Keycode::Num8 => self.keys[8] = 1,
                            Keycode::Num9 => self.keys[9] = 1,
                            _ => {}
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Num0 => self.keys[0] = 0,
                            Keycode::Num1 => self.keys[1] = 0,
                            Keycode::Num2 => self.keys[2] = 0,
                            Keycode::Num3 => self.keys[3] = 0,
                            Keycode::Num4 => self.keys[4] = 0,
                            Keycode::Num5 => self.keys[5] = 0,
                            Keycode::Num6 => self.keys[6] = 0,
                            Keycode::Num7 => self.keys[7] = 0,
                            Keycode::Num8 => self.keys[8] = 0,
                            Keycode::Num9 => self.keys[9] = 0,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            self.cycle();

            if self.refresh {
                self.render(&mut renderer);
                renderer.present();
                self.refresh = false;
            }

            let duration = time::Duration::from_millis(16);
            thread::sleep(duration);
        }
    }

    #[cfg(not(test))]
    fn render(&self, renderer: &mut sdl2::render::Renderer) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_position = (y * (DISPLAY_HEIGHT)) + x;
                let pixel = self.reg_gfx[pixel_position] as usize;
                let color = if self.reg_gfx[pixel] == 0 {
                    pixels::Color::RGB(0, 255, 0)
                } else {
                    pixels::Color::RGB(255, 255, 255)
                };

                let rectangle = sdl2::rect::Rect::new((x * PIXEL_SIZE) as i32,
                                                      (y * PIXEL_SIZE) as i32,
                                                      PIXEL_SIZE as u32,
                                                      PIXEL_SIZE as u32);
                renderer.set_draw_color(color);
                let _ = renderer.fill_rect(rectangle);
            }
        }
    }

    fn cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer - 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer = self.sound_timer - 1;
        }
    }

    fn fetch_opcode(&self) -> Opcode {
        let opcode = ((self.memory[self.program_counter as usize] as u16) << 8) +
                     (self.memory[self.program_counter as usize + 1] as u16);
        Opcode::new(opcode)
    }

    fn clear_screen(&mut self) {
        self.reg_gfx = [0; GFX_MEMORY_SIZE];
        self.refresh = true;
    }

    fn display(&mut self, x: usize, y: usize, height: u8) {
        self.reg_v[0xF] = 0x00;
        self.refresh = true;
        for y_line in 0..height {
            let memory_position = (self.reg_i + y_line as u16) as usize;
            let pixel = self.memory[memory_position];
            for x_line in 0..8 {
                if (pixel & (0x80 >> x_line)) != 0x00 {
                    let gfx_position = x + x_line + ((y + y_line as usize) * DISPLAY_WIDTH);
                    let current_pixel = self.reg_gfx[gfx_position];
                    if current_pixel == 0x01 {
                        self.reg_v[0xF] = 1;
                    }

                    self.reg_gfx[gfx_position] = self.reg_gfx[gfx_position] ^ 1;
                }
            }
        }
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        match opcode.category {
            0 => {
                match opcode.byte {
                    0xE0 => {
                        self.clear_screen();
                        self.program_counter = self.program_counter + 2;
                    }
                    0xEE => {
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.stack_pointer = self.stack_pointer - 1;
                        self.program_counter = self.program_counter + 2;
                    }
                    _ => panic!("Invalid OpCode"),
                }
            }
            1 => self.program_counter = opcode.address,
            2 => {
                self.stack_pointer = self.stack_pointer + 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = opcode.address;
            }
            3 => {
                if self.reg_v[opcode.x] == opcode.byte {
                    self.program_counter = self.program_counter + 4;
                } else {
                    self.program_counter = self.program_counter + 2;
                }
            }
            4 => {
                if self.reg_v[opcode.x] != opcode.byte {
                    self.program_counter = self.program_counter + 4;
                } else {
                    self.program_counter = self.program_counter + 2;
                }
            }
            5 => {
                if self.reg_v[opcode.x] == self.reg_v[opcode.y] {
                    self.program_counter = self.program_counter + 4;
                } else {
                    self.program_counter = self.program_counter + 2;
                }
            }
            6 => {
                self.reg_v[opcode.x] = opcode.byte;
                self.program_counter = self.program_counter + 2;
            }
            7 => {
                self.reg_v[opcode.x] = self.reg_v[opcode.x] + opcode.byte;
                self.program_counter = self.program_counter + 2;
            }
            8 => {
                match opcode.nibble {
                    0 => {
                        self.reg_v[opcode.x] = self.reg_v[opcode.y];
                        self.program_counter = self.program_counter + 2;
                    }
                    1 => {
                        self.reg_v[opcode.x] = self.reg_v[opcode.x] | self.reg_v[opcode.y];
                        self.program_counter = self.program_counter + 2;
                    }
                    2 => {
                        self.reg_v[opcode.x] = self.reg_v[opcode.x] & self.reg_v[opcode.y];
                        self.program_counter = self.program_counter + 2;
                    }
                    3 => {
                        self.reg_v[opcode.x] = self.reg_v[opcode.x] ^ self.reg_v[opcode.y];
                        self.program_counter = self.program_counter + 2;
                    }
                    4 => {
                        self.memory[0xF] = if opcode.y > (0xFF - opcode.x) { 1 } else { 0 };

                        self.reg_v[opcode.x] = self.reg_v[opcode.x]
                            .wrapping_add(self.reg_v[opcode.y]);
                        self.program_counter = self.program_counter + 2;
                    }
                    5 => {
                        self.memory[0xF] = if opcode.y > opcode.x { 1 } else { 0 };
                        self.reg_v[opcode.x] = self.reg_v[opcode.x]
                            .wrapping_sub(self.reg_v[opcode.y]);
                        self.program_counter = self.program_counter + 2;
                    }
                    6 => {
                        self.memory[0xF] = self.reg_v[opcode.x] & 0x01;
                        self.reg_v[opcode.x] = self.reg_v[opcode.x] >> 1;
                        self.program_counter = self.program_counter + 2;
                    }
                    7 => {
                        self.memory[0xF] = if opcode.x > opcode.y { 1 } else { 0 };
                        self.reg_v[opcode.x] = self.reg_v[opcode.y]
                            .wrapping_sub(self.reg_v[opcode.x]);
                        self.program_counter = self.program_counter + 2;
                    }
                    0xE => {
                        self.memory[0xF] = self.reg_v[opcode.x] & 0x80;
                        self.reg_v[opcode.x] = self.reg_v[opcode.x] << 1;
                        self.program_counter = self.program_counter + 2;
                    }
                    _ => panic!("Invalid OpCode"),
                }
            }
            9 => {
                if self.reg_v[opcode.x] != self.reg_v[opcode.y] {
                    self.program_counter = self.program_counter + 4;
                } else {
                    self.program_counter = self.program_counter + 2;
                }
            }
            0xA => {
                self.reg_i = opcode.address;
                self.program_counter = self.program_counter + 2;
            }
            0xB => {
                self.program_counter = opcode.address + self.reg_v[opcode.x] as u16;
                self.program_counter = self.program_counter + 2;
            }
            0xC => {
                self.reg_v[opcode.x] = rand::random::<u8>() & opcode.byte;
                self.program_counter = self.program_counter + 2;
            }
            0xD => {
                self.display(opcode.x, opcode.y, opcode.nibble);
                self.program_counter = self.program_counter + 2;
            }
            0xE => {
                match opcode.byte {
                    0x9E => {
                        if self.keys[self.reg_v[opcode.x] as usize] != 0 {
                            self.program_counter = self.program_counter + 4;
                        } else {
                            self.program_counter = self.program_counter + 2;
                        }
                    }
                    0xA1 => {
                        if self.keys[self.reg_v[opcode.x] as usize] == 0 {
                            self.program_counter = self.program_counter + 4;
                        } else {
                            self.program_counter = self.program_counter + 2;
                        }
                    }
                    _ => panic!("Invalid OpCode!"),
                }
            }
            0xF => {
                match opcode.byte {
                    0x07 => {
                        self.reg_v[opcode.x] = self.delay_timer;
                        self.program_counter = self.program_counter + 2;
                    }
                    0x0A => {
                        for index in 0..NUM_KEYS {
                            if self.keys[index] != 0x00 {
                                self.reg_v[opcode.x] = self.keys[index];
                                self.program_counter = self.program_counter + 2;
                                break;
                            }
                        }
                    }
                    0x15 => {
                        self.delay_timer = self.reg_v[opcode.x];
                        self.program_counter = self.program_counter + 2;
                    }
                    0x18 => {
                        self.sound_timer = self.reg_v[opcode.x];
                        self.program_counter = self.program_counter + 2;
                    }
                    0x1E => {
                        let i = self.reg_i + self.reg_v[opcode.x] as u16;
                        if i > 0xFFF {
                            self.reg_v[0xF] = 1;
                        } else {
                            self.reg_v[0xF] = 0;
                        }

                        self.reg_i = i;
                        self.program_counter = self.program_counter + 2;
                    }
                    0x29 => {
                        self.reg_i = self.reg_v[opcode.x] as u16 * 0x5;
                        self.program_counter = self.program_counter + 2;
                    }
                    0x33 => {
                        let x = self.reg_v[opcode.x];
                        self.memory[self.reg_i as usize] = x / 100;
                        self.memory[(self.reg_i + 1) as usize] = (x / 10) % 10;
                        self.memory[(self.reg_i + 2) as usize] = x % 100 % 10;
                        self.program_counter = self.program_counter + 2;
                    }
                    0x55 => {
                        for x in 0..opcode.x {
                            self.memory[self.reg_i as usize + x] = self.reg_v[x];
                        }

                        self.program_counter = self.program_counter + 2;
                    }
                    0x65 => {
                        for x in 0..opcode.x {
                            self.reg_v[x] = self.memory[self.reg_i as usize + x];
                        }

                        self.program_counter = self.program_counter + 2;
                    }
                    _ => panic!("Invalid OpCode"),
                }
            }
            _ => panic!("Invalid OpCode"),
        }
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn instruction_clear_display() {
        let rom = vec![0x00, 0xE0];

        let mut chip = ::Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_call() {
        let rom = vec![0x22, 0xFC];

        let mut chip = ::Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x02FC);
        assert_eq!(chip.stack_pointer, 0x0001);
        assert_eq!(chip.stack[chip.stack_pointer as usize], 0x200);
    }

    #[test]
    fn instruction_return() {
        let rom = vec![0x22, 0x04, 0x00, 0x00, 0x00, 0xEE];

        let mut chip = ::Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.stack_pointer, 0x0000);
    }

    #[test]
    fn instruction_jump() {
        let rom = vec![0x12, 0xFC];

        let mut chip = ::Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x02FC);
    }

    #[test]
    fn instruction_jump_equal() {
        let rom = vec![0x30, 0x15];

        let mut chip = ::Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v[0] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_jump_equal() {
        let rom = vec![0x30, 0x15];

        let mut chip = ::Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v[0] = 0x14;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }
}