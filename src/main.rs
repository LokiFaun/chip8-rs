use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    if let Some(rom_file_name) = env::args().nth(1) {
        println!("ROM: {}", rom_file_name);
        if let Some(rom) = read_rom(rom_file_name) {
            println!("{:?}", rom);
            return;
        } else {
            println!("Could not read ROM file!");
        }
    }

    println!("Usage: chip8 <rom>");
}

fn read_rom<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut file_buffer = Vec::new();
            match file.read_to_end(&mut file_buffer) {
                Ok(_) => Some(file_buffer),
                Err(ex) => {
                    println!("Could not read from file: {}", ex);
                    None
                }
            }
        }
        Err(ex) => {
            println!("Could not open file: {}", ex);
            None
        }
    }
}
