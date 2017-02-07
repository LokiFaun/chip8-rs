use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    if let Some(rom_file_name) = env::args().nth(1) {
        println!("ROM: {}", rom_file_name);
        match read_rom(rom_file_name) {
            Ok(rom) => println!("{:?}", rom),
            Err(err) => println!("Could not read ROM: {}", err),
        };
    }

    println!("Usage: chip8 <rom>");
}

fn read_rom<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = try!(fs::File::open(path));
    let mut file_buffer = Vec::new();
    try!(file.read_to_end(&mut file_buffer));
    Ok(file_buffer)
}
