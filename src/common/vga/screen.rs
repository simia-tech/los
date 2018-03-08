
use core::result;
use super::super::vga;

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
}

pub type Result<T> = result::Result<T, Error>;

pub trait Screen {
    fn new() -> Self;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn set(&mut self, usize, usize, u8, vga::Color, vga::Color) -> Result<()>;
    fn get(&self, usize, usize) -> Result<(u8, vga::Color, vga::Color)>;
}
