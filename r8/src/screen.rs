pub struct Screen {
    width: u32,
    height: u32,
    size: u32,
    screen: [[u32; 32]; 64],
}

impl Screen {
    pub const fn new() -> Screen {
        Screen {
            width: 64,
            height: 32,
            size: 64 * 32,
            screen: [[0x000; 32]; 64], // todo: not sure if 2d is a good idea
        }
    }
}
