use r8::R8;

pub struct CPU {
    rom: Vec<char>,
    r8: R8,
}

impl CPU {
    pub fn new(rom: Vec<char>) -> CPU {
        CPU {
            rom: rom,
            r8: R8::new(),
        }
    }

    fn load_rom(&self) {
        // todo: implement rom loading, look at my project CPP8. Chip8.cpp > LoadRom()
        // rom stored in self.rom
    }
}
