#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rand;
extern crate sdl2;
extern crate timer;
extern crate chrono;

mod opcode;
mod chip8;
mod stack;
mod error;

#[cfg(not(test))]
fn main() {
    use std::env;

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
            if let Err(err) = chip.run() {
                match err {
                    error::Chip8Error::Message(msg) => {
                        println!("Error running chip8: {}", msg);
                    }
                    _ => {
                        println!("Error running chip8: {:?}", err);
                    }
                }
            };
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