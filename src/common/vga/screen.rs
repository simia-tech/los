
use super::super::vga;

pub trait Screen {
    fn new() -> Self;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn set(&mut self, usize, usize, u8, vga::Color, vga::Color);
    fn get(&self, usize, usize) -> (u8, vga::Color, vga::Color);
}
