extern crate rand;
extern crate sdl2;
extern crate timer;
extern crate chrono;

mod opcode;
mod chip8;

fn test_timer() {
    use std::thread;
    use std::sync::{Arc, Mutex};
    let timer = timer::Timer::new();
    let count = Arc::new(Mutex::new(0));
    let guard = {
        let count = count.clone();
        // Instructions per second: 840 => 1sec / 840 = 1190µs
        timer.schedule_repeating(chrono::Duration::nanoseconds(1000000000 / 840), move || {
            *count.lock().unwrap() += 1;
        })
    };

    thread::sleep(std::time::Duration::new(1, 0));
    let count_result = *count.lock().unwrap();
    println!("{0} instruction executions!", count_result);
    drop(guard);
}

#[cfg(not(test))]
fn main() {
    use std::env;

    test_timer();

    if let Some(rom_file_name) = env::args().nth(1) {
        println!("Reading ROM: {}...", rom_file_name);
        let rom = utils::read_binary(rom_file_name).unwrap_or_else(|err| {
            println!("Could not read ROM: {}!", err);
            Vec::new()
        });

        if !rom.is_empty() {
            println!("Starting CHIP-8 emulator...");
            let mut chip = chip8::Chip8::new();
            chip.initialize();
            chip.load_rom(rom);
            match chip.run() {
                Err(err) => {
                    match err {
                        chip8::Chip8Error::Message(msg) => {
                            println!("Error running chip8: {}", msg);
                        }
                        _ => {
                            println!("Error running chip8: {:?}", err);
                        }
                    }
                }
                _ => {}
            }
        }
    } else {
        println!("Usage: chip8 <rom>");
    }
}

#[cfg(not(test))]
mod utils {
    use std::io;
    use std::path::Path;

    pub fn read_binary<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        use std::fs;
        use std::io::Read;

        let mut file = try!(fs::File::open(path));
        let mut file_buffer = Vec::new();
        try!(file.read_to_end(&mut file_buffer));
        Ok(file_buffer)
    }
}