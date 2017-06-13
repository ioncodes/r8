pub struct Registers {
    I: u32,
    V: [u32; 16],
}

impl Registers {
    pub const fn new() -> Registers {
        Registers {
            I: 0x000,
            V: [0x000; 16],
        }
    }
}
