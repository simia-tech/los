
use core::ptr::Unique;
use volatile::Volatile;
use common::vga;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Screen {
    buffer: Unique<Buffer>,
}

impl vga::Screen for Screen {

    fn new() -> Screen {
        Screen {
            buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
        }
    }

    fn height(&self) -> usize {
        BUFFER_HEIGHT
    }

    fn width(&self) -> usize {
        BUFFER_WIDTH
    }

    fn set(&mut self, x: usize, y: usize, c: u8, fg: vga::Color, bg: vga::Color) -> vga::screen::Result<()> {
        if x >= self.width() || y >= self.height() {
            return Err(vga::screen::Error::OutOfBounds);
        }

        unsafe { self.buffer.as_mut() }.chars[y][x].write(ScreenChar {
            ascii_character: c,
            color_code: ColorCode::new(fg, bg),
        });

        Ok(())
    }

    fn get(&self, x: usize, y: usize) -> vga::screen::Result<(u8, vga::Color, vga::Color)> {
        if x >= self.width() || y >= self.height() {
            return Err(vga::screen::Error::OutOfBounds);
        }

        let sc = unsafe { self.buffer.as_ref() }.chars[y][x].read();
        Ok((sc.ascii_character, sc.color_code.foreground(), sc.color_code.background()))
    }

}

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: vga::Color, background: vga::Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }

    fn foreground(self) -> vga::Color {
        vga::Color::from(self.0 & 0x0f)
    }

    fn background(self) -> vga::Color {
        vga::Color::from((self.0 >> 4) & 0x0f)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
