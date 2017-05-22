use super::*;

use std::sync::{Arc, Mutex, Condvar};

use gfx::GfxMemory;
use keyboard::Keyboard;
use error::Chip8Error;

const PIXEL_SIZE: usize = 20;

pub struct Renderer {
    gfx: Arc<Mutex<GfxMemory>>,
    keys: Arc<Mutex<Keyboard>>,
    shutdown: Arc<(Mutex<bool>, Condvar)>,
    started: Arc<(Mutex<bool>, Condvar)>,
}

impl Renderer {
    pub fn new(gfx: Arc<Mutex<GfxMemory>>,
               keyboard: Arc<Mutex<Keyboard>>,
               shutdown: Arc<(Mutex<bool>, Condvar)>,
               started: Arc<(Mutex<bool>, Condvar)>)
               -> Renderer {
        Renderer {
            gfx: gfx,
            keys: keyboard,
            shutdown: shutdown,
            started: started,
        }
    }

    pub fn start(gfx: Arc<Mutex<GfxMemory>>,
                 keyboard: Arc<Mutex<Keyboard>>,
                 shutdown: Arc<(Mutex<bool>, Condvar)>,
                 started: Arc<(Mutex<bool>, Condvar)>) {
        let renderer = Renderer::new(gfx, keyboard, shutdown, started);
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
        let window = try!(video_subsys
                              .window("chip8",
                                      (DISPLAY_WIDTH * PIXEL_SIZE) as u32,
                                      (DISPLAY_HEIGHT * PIXEL_SIZE) as u32)
                              .position_centered()
                              .opengl()
                              .build());

        let mut renderer = try!(window.renderer().build());
        renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        let &(ref lock, ref condition) = &*self.started;
        {
            let mut start = lock.lock().unwrap();
            *start = true;
            condition.notify_all();
        }

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

            try!(self.render(&mut renderer));
            renderer.present();
        }

        let &(ref lock, ref condition) = &*self.shutdown;
        let mut stopped = lock.lock().unwrap();
        *stopped = true;
        condition.notify_all();
        Ok(())
    }

    fn render<'a>(&self, renderer: &mut sdl2::render::Renderer<'a>) -> Result<(), String> {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let index = (y * DISPLAY_WIDTH) + x;
                let color = if self.gfx.as_ref().lock().unwrap()[index] == 0 {
                    sdl2::pixels::Color::RGB(0, 0, 0)
                } else {
                    sdl2::pixels::Color::RGB(255, 255, 255)
                };

                let rectangle = sdl2::rect::Rect::new((x * PIXEL_SIZE) as i32,
                                                      (y * PIXEL_SIZE) as i32,
                                                      PIXEL_SIZE as u32,
                                                      PIXEL_SIZE as u32);
                renderer.set_draw_color(color);
                try!(renderer.fill_rect(rectangle));
            }
        }

        Ok(())
    }

    fn key_press(&self, keycode: sdl2::keyboard::Keycode, up: u8) {
        use sdl2::keyboard::Keycode;
        let mut keys = self.keys.lock().unwrap();
        match keycode {
            Keycode::Num1 => keys[0x1] = up,
            Keycode::Num2 => keys[0x2] = up,
            Keycode::Num3 => keys[0x3] = up,
            Keycode::Num4 => keys[0xC] = up,
            Keycode::Q => keys[0x4] = up,
            Keycode::W => keys[0x5] = up,
            Keycode::E => keys[0x6] = up,
            Keycode::R => keys[0xD] = up,
            Keycode::A => keys[0x7] = up,
            Keycode::S => keys[0x8] = up,
            Keycode::D => keys[0x9] = up,
            Keycode::F => keys[0xE] = up,
            Keycode::Y => keys[0xA] = up,
            Keycode::X => keys[0x0] = up,
            Keycode::C => keys[0xB] = up,
            Keycode::V => keys[0xF] = up,
            _ => {}
        }
    }
}

