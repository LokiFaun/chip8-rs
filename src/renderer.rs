use super::*;

#[cfg(not(test))]
use sdl2::pixels;

use std::sync::{Arc, Mutex};

use gfx::GfxMemory;
use keyboard::Keyboard;
use error::Chip8Error;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
const PIXEL_SIZE: usize = 20;

pub struct Renderer {
    gfx: Arc<Mutex<GfxMemory>>,
    keys: Arc<Mutex<Keyboard>>,
}

impl Renderer {
    pub fn new(gfx: Arc<Mutex<GfxMemory>>, keyboard: Arc<Mutex<Keyboard>>) -> Renderer {
        Renderer {
            gfx: gfx,
            keys: keyboard,
        }
    }

    pub fn start(gfx: Arc<Mutex<GfxMemory>>, keyboard: Arc<Mutex<Keyboard>>) {
        let renderer = Renderer::new(gfx, keyboard);
        if let Err(err) = renderer.run() {
            match err {
                error::Chip8Error::Message(msg) => {
                    println!("Error rendering: {}", msg);
                }
                _ => {
                    println!("Error rendering: {:?}", err);
                }
            }
        }
    }

    pub fn run(&self) -> Result<(), Chip8Error> {
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
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            _ => self.key_press(keycode, 1),
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Escape => break 'main,
                            _ => self.key_press(keycode, 0),
                        }
                    }
                    _ => {}
                }
            }


            for y in 0..DISPLAY_HEIGHT {
                for x in 0..DISPLAY_WIDTH {
                    let index = (y * DISPLAY_WIDTH) + x;
                    let color = if self.gfx.as_ref().lock().unwrap()[index] == 0 {
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
            renderer.present();
        }

        Ok(())
    }

    fn key_press(&self, keycode: sdl2::keyboard::Keycode, up: u8) {
        use sdl2::keyboard::Keycode;
        match keycode {
            Keycode::Num1 => self.keys.lock().unwrap()[0x1] = up,
            Keycode::Num2 => self.keys.lock().unwrap()[0x2] = up,
            Keycode::Num3 => self.keys.lock().unwrap()[0x3] = up,
            Keycode::Num4 => self.keys.lock().unwrap()[0xC] = up,
            Keycode::Q => self.keys.lock().unwrap()[0x4] = up,
            Keycode::W => self.keys.lock().unwrap()[0x5] = up,
            Keycode::E => self.keys.lock().unwrap()[0x6] = up,
            Keycode::R => self.keys.lock().unwrap()[0xD] = up,
            Keycode::A => self.keys.lock().unwrap()[0x7] = up,
            Keycode::S => self.keys.lock().unwrap()[0x8] = up,
            Keycode::D => self.keys.lock().unwrap()[0x9] = up,
            Keycode::F => self.keys.lock().unwrap()[0xE] = up,
            Keycode::Y => self.keys.lock().unwrap()[0xA] = up,
            Keycode::X => self.keys.lock().unwrap()[0x0] = up,
            Keycode::C => self.keys.lock().unwrap()[0xB] = up,
            Keycode::V => self.keys.lock().unwrap()[0xF] = up,

            _ => {}
        }
    }
}
