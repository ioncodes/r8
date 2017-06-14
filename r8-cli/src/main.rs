extern crate r8;

use r8::cpu::CPU;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    //let mut rom_path = env::args().nth(1).unwrap();
    let mut rom_path = "../roms/BLITZ".to_owned();
    let mut rom_file = File::open(rom_path).unwrap();
    let mut buffer = Vec::new();
    rom_file.read_to_end(&mut buffer);
    let mut rom_buffer = Vec::<char>::new();
    for byte in buffer {
        rom_buffer.push(byte as char);
    }
    let cpu = CPU::new(rom_buffer);
    cpu.load_rom(); // load the rom
}
