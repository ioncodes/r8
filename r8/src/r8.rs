use screen::Screen;
use registers::Registers;

pub const MEMORY_SIZE: u32 = 0x1000;
pub const ENTRY_POINT: u32 = 0x200;
pub const FONT_SIZE: u32 = 0x50;
pub const SCREEN: Screen = Screen::new();

pub struct R8 {
    pub program_counter: u32,
    pub stack_pointer: u32,
    pub registers: Registers,
}
