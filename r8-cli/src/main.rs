extern crate r8;

use r8::cpu;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let rom_path = env::args().nth(1).unwrap();
    let mut rom_file = File::open(rom_path).unwrap();
    let mut buffer = Vec::new();
    rom_file.read_to_end(&mut buffer);
    let mut rom_buffer = Vec::<char>::new();
    for byte in buffer {
        rom_buffer.push(byte as char);
    }
    let r = cpu::CPU::new(rom_buffer);
}
