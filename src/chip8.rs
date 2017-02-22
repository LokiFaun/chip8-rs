use super::*;

#[cfg(not(test))]
use sdl2::pixels;

use std::sync::{Arc, Mutex};

use opcode::Opcode;
use error::Chip8Error;
use gfx::GfxMemory;
use register::Register;

#[cfg(not(test))]
const PIXEL_SIZE: usize = 20;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
const MEMORY_SIZE: usize = 4096;
const NUM_KEYS: usize = 16;
const PROGRAM_START: usize = 0x200;
const FONT_SET_SIZE: usize = 80;
const INSTRUCTIONS_PER_SECOND: i64 = 840;
const NANO_SECONDS_PER_SECOND: i64 = 1000 * 1000 * 1000;
const FONT_SET: [u8; FONT_SET_SIZE] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
     0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
     0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
     0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
     0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
     0xF0, 0x80, 0xF0, 0x80, 0x80];

pub struct Chip8 {
    reg_v: Arc<Mutex<Register>>,
    reg_gfx: Arc<Mutex<GfxMemory>>,
    stack: Arc<Mutex<Box<stack::IStack>>>,

    memory: [u8; MEMORY_SIZE],

    keys: [u8; NUM_KEYS],

    program_counter: u16,

    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            reg_gfx: Arc::new(Mutex::new(GfxMemory::new())),
            reg_v: Arc::new(Mutex::new(Register::new())),
            stack: Arc::new(Mutex::new(Box::new(stack::Stack::new()))),
            keys: [0; NUM_KEYS],
            memory: [0; MEMORY_SIZE],
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.reg_v.as_ref().lock().unwrap().clear();
        self.reg_gfx.as_ref().lock().unwrap().clear();
        self.program_counter = PROGRAM_START as u16;
        self.memory = [0; MEMORY_SIZE];
        self.keys = [0; NUM_KEYS];
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
    pub fn run(&mut self) -> Result<(), Chip8Error> {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        let sdl_context = try!(sdl2::init());
        let video_subsys = try!(sdl_context.video());
        let window = try!(video_subsys.window("chip8",
                    (DISPLAY_WIDTH * PIXEL_SIZE) as u32,
                    (DISPLAY_HEIGHT * PIXEL_SIZE) as u32)
            .position_centered()
            .opengl()
            .build());

        let mut renderer = try!(window.renderer().build());
        renderer.set_draw_color(pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        let mut events = try!(sdl_context.event_pump());
        'main: loop {
            let start = std::time::Instant::now();
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            Keycode::Num1 => self.keys[0x1] = 1,
                            Keycode::Num2 => self.keys[0x2] = 1,
                            Keycode::Num3 => self.keys[0x3] = 1,
                            Keycode::Num4 => self.keys[0xC] = 1,
                            Keycode::Q => self.keys[0x4] = 1,
                            Keycode::W => self.keys[0x5] = 1,
                            Keycode::E => self.keys[0x6] = 1,
                            Keycode::R => self.keys[0xD] = 1,
                            Keycode::A => self.keys[0x7] = 1,
                            Keycode::S => self.keys[0x8] = 1,
                            Keycode::D => self.keys[0x9] = 1,
                            Keycode::F => self.keys[0xE] = 1,
                            Keycode::Y => self.keys[0xA] = 1,
                            Keycode::X => self.keys[0x0] = 1,
                            Keycode::C => self.keys[0xB] = 1,
                            Keycode::V => self.keys[0xF] = 1,

                            _ => {}
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            Keycode::Num1 => self.keys[0x1] = 0,
                            Keycode::Num2 => self.keys[0x2] = 0,
                            Keycode::Num3 => self.keys[0x3] = 0,
                            Keycode::Num4 => self.keys[0xC] = 0,
                            Keycode::Q => self.keys[0x4] = 0,
                            Keycode::W => self.keys[0x5] = 0,
                            Keycode::E => self.keys[0x6] = 0,
                            Keycode::R => self.keys[0xD] = 0,
                            Keycode::A => self.keys[0x7] = 0,
                            Keycode::S => self.keys[0x8] = 0,
                            Keycode::D => self.keys[0x9] = 0,
                            Keycode::F => self.keys[0xE] = 0,
                            Keycode::Y => self.keys[0xA] = 0,
                            Keycode::X => self.keys[0x0] = 0,
                            Keycode::C => self.keys[0xB] = 0,
                            Keycode::V => self.keys[0xF] = 0,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            self.cycle();

            self.render(&mut renderer);
            renderer.present();

            let elapsed = start.elapsed();
            let elapsed_duration = chrono::Duration::from_std(elapsed).unwrap();
            let cycle_time = chrono::Duration::nanoseconds(NANO_SECONDS_PER_SECOND /
                                                           INSTRUCTIONS_PER_SECOND);
            if cycle_time > elapsed_duration {
                let sleep_time = cycle_time - elapsed_duration;
                let sleep_duration = sleep_time.to_std().unwrap();
                std::thread::sleep(sleep_duration);
            }
        }

        Ok(())
    }

    #[cfg(not(test))]
    fn render(&self, renderer: &mut sdl2::render::Renderer) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let index = (y * DISPLAY_WIDTH) + x;
                let color = if self.reg_gfx.as_ref().lock().unwrap()[index] == 0 {
                    pixels::Color::RGB(0, 0, 0)
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
        println!("Executing Opcode: {}", opcode);
        self.execute_opcode(opcode);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn fetch_opcode(&self) -> Opcode {
        let index = self.program_counter as usize;
        let a = self.memory[index] as u16;
        let b = self.memory[index + 1] as u16;
        let opcode = (a << 8) + b;
        Opcode::new(opcode)
    }

    fn display(&mut self, x: usize, y: usize, height: u8) {
        self.reg_v.as_ref().lock().unwrap()[0xF] = 0x00;
        let reg_v = self.reg_v.clone();
        let reg_i = reg_v.lock().unwrap().reg_i;
        let reg_gfx = self.reg_gfx.clone();
        for y_line in 0..height {
            let memory_position = (reg_i + y_line as u16) as usize;
            let pixel = self.memory[memory_position];
            for x_line in 0..8 {
                if (pixel & (0x80 >> x_line)) != 0x00 {
                    let x = reg_v.lock().unwrap()[x];
                    let y = reg_v.lock().unwrap()[y];
                    let gfx_position = (x as usize + x_line +
                                        ((y as usize + y_line as usize) * DISPLAY_WIDTH)) %
                                       gfx::GFX_MEMORY_SIZE;
                    let current_pixel = reg_gfx.lock().unwrap()[gfx_position as usize];
                    reg_v.lock().unwrap()[0xF] = current_pixel & 0x01;

                    reg_gfx.lock()
                        .unwrap()[gfx_position as usize] = !current_pixel;
                }
            }
        }
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        let reg_v = self.reg_v.clone();
        let reg_gfx = self.reg_gfx.clone();
        match opcode.category {
            0 => {
                match opcode.byte {
                    0xE0 => {
                        reg_gfx.lock().unwrap().clear();
                        self.program_counter += 2;
                    }
                    0xEE => {
                        self.program_counter = self.stack.lock().unwrap().pop();
                        self.program_counter += 2;
                    }
                    _ => {}
                }
            }
            1 => self.program_counter = opcode.address,
            2 => {
                self.stack.lock().unwrap().push(self.program_counter);
                self.program_counter = opcode.address;
            }
            3 => {
                if reg_v.lock().unwrap()[opcode.x] == opcode.byte {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            4 => {
                if reg_v.lock().unwrap()[opcode.x] != opcode.byte {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            5 => {
                let x = reg_v.lock().unwrap()[opcode.x];
                let y = reg_v.lock().unwrap()[opcode.y];
                if x == y {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            6 => {
                reg_v.lock().unwrap()[opcode.x] = opcode.byte;
                self.program_counter += 2;
            }
            7 => {
                let x = reg_v.lock().unwrap()[opcode.x];
                reg_v.lock().unwrap()[opcode.x] = x.wrapping_add(opcode.byte);
                self.program_counter += 2;
            }
            8 => {
                match opcode.nibble {
                    0 => {
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[opcode.x] = y;
                        self.program_counter += 2;
                    }
                    1 => {
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[opcode.x] |= y;
                        self.program_counter += 2;
                    }
                    2 => {
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[opcode.x] &= y;
                        self.program_counter += 2;
                    }
                    3 => {
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[opcode.x] ^= y;
                        self.program_counter += 2;
                    }
                    4 => {
                        let x = reg_v.lock().unwrap()[opcode.x];
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[0xF] = ((x as u16 + y as u16) > 0xFF) as u8;

                        reg_v.lock().unwrap()[opcode.x] = x.wrapping_add(y);
                        self.program_counter += 2;
                    }
                    5 => {
                        let x = reg_v.lock().unwrap()[opcode.x];
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[0xF] = (x >= y) as u8;
                        reg_v.lock().unwrap()[opcode.x] = x.wrapping_sub(y);
                        self.program_counter += 2;
                    }
                    6 => {
                        let carry = reg_v.lock().unwrap()[opcode.x] & 0x01;
                        reg_v.lock().unwrap()[0xF] = carry;
                        reg_v.lock().unwrap()[opcode.x] >>= 1;
                        self.program_counter += 2;
                    }
                    7 => {
                        let x = reg_v.lock().unwrap()[opcode.x];
                        let y = reg_v.lock().unwrap()[opcode.y];
                        reg_v.lock().unwrap()[0xF] = (y >= x) as u8;
                        reg_v.lock().unwrap()[opcode.x] = y.wrapping_sub(x);
                        self.program_counter += 2;
                    }
                    0xE => {
                        let carry = (reg_v.lock().unwrap()[opcode.x] & 0x80) >> 7;
                        reg_v.lock().unwrap()[0xF] = carry;
                        reg_v.lock().unwrap()[opcode.x] <<= 1;
                        self.program_counter += 2;
                    }
                    _ => {}
                }
            }
            9 => {
                let x = reg_v.lock().unwrap()[opcode.x];
                let y = reg_v.lock().unwrap()[opcode.y];
                if x != y {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0xA => {
                reg_v.lock().unwrap().reg_i = opcode.address;
                self.program_counter += 2;
            }
            0xB => {
                self.program_counter = opcode.address + reg_v.lock().unwrap()[0] as u16;
            }
            0xC => {
                reg_v.lock().unwrap()[opcode.x] = rand::random::<u8>() & opcode.byte;
                self.program_counter += 2;
            }
            0xD => {
                self.display(opcode.x, opcode.y, opcode.nibble);
                self.program_counter += 2;
            }
            0xE => {
                match opcode.byte {
                    0x9E => {
                        if self.keys[reg_v.lock().unwrap()[opcode.x] as usize] != 0 {
                            self.program_counter += 4;
                        } else {
                            self.program_counter += 2;
                        }
                    }
                    0xA1 => {
                        if self.keys[reg_v.lock().unwrap()[opcode.x] as usize] == 0 {
                            self.program_counter += 4;
                        } else {
                            self.program_counter += 2;
                        }
                    }
                    _ => panic!("Invalid OpCode!"),
                }
            }
            0xF => {
                match opcode.byte {
                    0x07 => {
                        reg_v.lock().unwrap()[opcode.x] = self.delay_timer;
                        self.program_counter += 2;
                    }
                    0x0A => {
                        for index in 0..NUM_KEYS {
                            if self.keys[index] != 0x00 {
                                reg_v.lock().unwrap()[opcode.x] = index as u8;
                                self.program_counter += 2;
                                break;
                            }
                        }
                    }
                    0x15 => {
                        self.delay_timer = reg_v.lock().unwrap()[opcode.x];
                        self.program_counter += 2;
                    }
                    0x18 => {
                        self.sound_timer = reg_v.lock().unwrap()[opcode.x];
                        self.program_counter += 2;
                    }
                    0x1E => {
                        let reg_i = reg_v.lock().unwrap().reg_i;
                        let x = reg_v.lock().unwrap()[opcode.x];
                        let i = reg_i + x as u16;
                        if i > 0xFFF {
                            reg_v.lock().unwrap()[0xF] = 1;
                        } else {
                            reg_v.lock().unwrap()[0xF] = 0;
                        }

                        reg_v.lock().unwrap().reg_i = i;
                        self.program_counter += 2;
                    }
                    0x29 => {
                        let x = reg_v.lock().unwrap()[opcode.x];
                        reg_v.lock().unwrap().reg_i = x as u16 * 0x5;
                        self.program_counter += 2;
                    }
                    0x33 => {
                        let x = reg_v.lock().unwrap()[opcode.x];
                        self.memory[reg_v.lock().unwrap().reg_i as usize] = x / 100;
                        self.memory[(reg_v.lock().unwrap().reg_i + 1) as usize] = (x / 10) % 10;
                        self.memory[(reg_v.lock().unwrap().reg_i + 2) as usize] = x % 100 % 10;
                        self.program_counter += 2;
                    }
                    0x55 => {
                        for x in 0..(opcode.x + 1) {
                            let reg_i = reg_v.lock().unwrap().reg_i;
                            self.memory[reg_i as usize + x] = reg_v.lock().unwrap()[x];
                        }

                        self.program_counter += 2;
                    }
                    0x65 => {
                        for x in 0..(opcode.x + 1) {
                            let reg_i = reg_v.lock().unwrap().reg_i;
                            reg_v.lock().unwrap()[x] = self.memory[reg_i as usize + x];
                        }

                        self.program_counter += 2;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_clear_display() {
        let rom = vec![0x00, 0xE0];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_call() {
        let rom = vec![0x22, 0xFC];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x02FC);
        assert_eq!(chip.stack.lock().unwrap().get_pointer(), 0x0001);
        assert_eq!(chip.stack.lock().unwrap().get(), 0x200);
    }

    #[test]
    fn instruction_return() {
        let rom = vec![0x22, 0x04, 0x00, 0x00, 0x00, 0xEE];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.stack.lock().unwrap().get_pointer(), 0x0000);
    }

    #[test]
    fn instruction_jump() {
        let rom = vec![0x12, 0xFC];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x02FC);
    }

    #[test]
    fn instruction_jump_equal() {
        let rom = vec![0x30, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_jump_equal() {
        let rom = vec![0x30, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x14;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_jump_not_equal() {
        let rom = vec![0x40, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x14;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_jump_not_equal() {
        let rom = vec![0x40, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_jump_equal_regs() {
        let rom = vec![0x50, 0x10];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x14;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x14;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_jump_equal_regs() {
        let rom = vec![0x50, 0x10];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x14;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_set_reg() {
        let rom = vec![0x60, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x15);
    }

    #[test]
    fn instruction_add_reg() {
        let rom = vec![0x70, 0x10];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x25);
    }

    #[test]
    fn instruction_assign() {
        let rom = vec![0x80, 0x10];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x15);
    }

    #[test]
    fn instruction_or() {
        let rom = vec![0x80, 0x11];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x0F;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0xF0;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0xFF);
    }

    #[test]
    fn instruction_and() {
        let rom = vec![0x80, 0x12];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x0F;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0xF0;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x00);
    }

    #[test]
    fn instruction_xor() {
        let rom = vec![0x80, 0x13];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x35;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x20);
    }

    #[test]
    fn instruction_add_carry() {
        let rom = vec![0x80, 0x14];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0xA5;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0xA5;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x4A);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x01);
    }

    #[test]
    fn instruction_add_no_carry() {
        let rom = vec![0x80, 0x14];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x10;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x25);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x0);
    }

    #[test]
    fn instruction_sub_no_carry() {
        let rom = vec![0x80, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x10;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0xFB);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x00);
    }

    #[test]
    fn instruction_sub_carry() {
        let rom = vec![0x80, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x10;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x05);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x1);
    }

    #[test]
    fn instruction_rshift_carry() {
        let rom = vec![0x80, 0x06];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x01;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x00);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x01);
    }

    #[test]
    fn instruction_rshift_no_carry() {
        let rom = vec![0x80, 0x06];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x02;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x01);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x0);
    }

    #[test]
    fn instruction_sub_regs_no_carry() {
        let rom = vec![0x80, 0x17];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x15;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x10;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0xFB);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x00);
    }

    #[test]
    fn instruction_sub_regs_carry() {
        let rom = vec![0x80, 0x17];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x10;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x05);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x01);
    }

    #[test]
    fn instruction_lshift_carry() {
        let rom = vec![0x80, 0x0E];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x80;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x00);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x01);
    }

    #[test]
    fn instruction_lshift_no_carry() {
        let rom = vec![0x80, 0x0E];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x01;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x02);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0xF], 0x0);
    }

    #[test]
    fn instruction_jump_not_equal_regs() {
        let rom = vec![0x90, 0x10];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x14;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x15;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_jump_not_equal_regs() {
        let rom = vec![0x90, 0x10];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x14;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x14;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_set_i() {
        let rom = vec![0xA1, 0x23];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap().reg_i, 0x0123);
    }

    #[test]
    fn instruction_jump_reg() {
        let rom = vec![0xB1, 0x23];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x10;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0133);
    }

    #[test]
    fn instruction_key_equal() {
        let rom = vec![0xE0, 0x9E];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x3;
        chip.keys[3] = 0x1;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_key_equal() {
        let rom = vec![0xE0, 0x9E];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x3;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_key_not_equal() {
        let rom = vec![0xE0, 0xA1];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x3;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0204);
    }

    #[test]
    fn instruction_not_key_not_equal() {
        let rom = vec![0xE0, 0xA1];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x3;
        chip.keys[3] = 0x1;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_get_timer() {
        let rom = vec![0xF0, 0x07];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.delay_timer = 0x12;
        chip.cycle();

        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x12);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_key_pressed() {
        let rom = vec![0xF0, 0x0A];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);

        chip.cycle();
        assert_eq!(chip.program_counter, 0x0200);

        chip.keys[5] = 0x1;
        chip.cycle();

        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x05);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_set_timer() {
        let rom = vec![0xF0, 0x15];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x12;
        chip.cycle();

        assert_eq!(chip.delay_timer, 0x11);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_set_sound() {
        let rom = vec![0xF0, 0x18];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x12;
        chip.cycle();

        assert_eq!(chip.sound_timer, 0x11);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_add_i() {
        let rom = vec![0xF0, 0x1E];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x12;
        chip.cycle();

        assert_eq!(chip.reg_v.as_ref().lock().unwrap().reg_i, 0x0012);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_sprite_addr() {
        let rom = vec![0xF0, 0x29];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x1;
        chip.cycle();

        assert_eq!(chip.reg_v.as_ref().lock().unwrap().reg_i, 0x0005);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_bcd() {
        let rom = vec![0xF0, 0x33];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0xF3;
        chip.reg_v.as_ref().lock().unwrap().reg_i = 0x0500;
        chip.cycle();

        assert_eq!(chip.memory[0x0500], 2);
        assert_eq!(chip.memory[0x0501], 4);
        assert_eq!(chip.memory[0x0502], 3);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_store() {
        let rom = vec![0xF2, 0x55];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap()[0] = 0x12;
        chip.reg_v.as_ref().lock().unwrap()[1] = 0x34;
        chip.reg_v.as_ref().lock().unwrap()[2] = 0x56;
        chip.reg_v.as_ref().lock().unwrap().reg_i = 0x500;
        chip.cycle();

        assert_eq!(chip.memory[0x0500], 0x12);
        assert_eq!(chip.memory[0x0501], 0x34);
        assert_eq!(chip.memory[0x0502], 0x56);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_load() {
        let rom = vec![0xF2, 0x65];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.memory[0x0500] = 0x12;
        chip.memory[0x0501] = 0x34;
        chip.memory[0x0502] = 0x56;
        chip.reg_v.as_ref().lock().unwrap().reg_i = 0x500;
        chip.cycle();

        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[0], 0x12);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[1], 0x34);
        assert_eq!(chip.reg_v.as_ref().lock().unwrap()[2], 0x56);
        assert_eq!(chip.program_counter, 0x0202);
    }

    #[test]
    fn instruction_display() {
        let rom = vec![0xD0, 0x05];

        let mut chip = Chip8::new();
        chip.initialize();
        chip.load_rom(rom);
        chip.reg_v.as_ref().lock().unwrap().reg_i = 0x0000;
        chip.cycle();

        assert_eq!(chip.program_counter, 0x0202);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[0], 0xFF);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[1], 0xFF);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[2], 0xFF);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[3], 0xFF);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[4], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[5], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[6], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[7], 0x00);

        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 0], 0xFF);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 1], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 2], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 3], 0xFF);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 4], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 5], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 6], 0x00);
        assert_eq!(chip.reg_gfx.as_ref().lock().unwrap()[64 + 7], 0x00);
    }
}