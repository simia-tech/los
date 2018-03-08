
use core::fmt;
use core::result;
use super::super::vga;

pub struct Console<T: vga::Screen> {
    screen: T,

    current_x: usize,
    foreground: vga::Color,
    background: vga::Color,
}

impl<T: vga::Screen> Console<T> {

    pub fn new(screen: T) -> Self {
        Console {
            screen: screen,
            current_x: 0,
            foreground: vga::Color::Green,
            background: vga::Color::Black,
        }
    }

    pub fn write_byte(&mut self, byte: u8) -> Result<()> {
        match byte {
            b'\n' => Ok(try!(self.new_line())),
            byte => {
                let screen_height = self.screen.height();
                let screen_width = self.screen.width();

                if self.current_x >= screen_width {
                    try!(self.new_line());
                }

                let y = screen_height - 1;
                let x = self.current_x;

                try!(self.screen.set(x, y, byte, self.foreground, self.background));
                self.current_x += 1;

                Ok(())
            }
        }
    }

    fn new_line(&mut self) -> Result<()> {
        let screen_height = self.screen.height();
        let screen_width = self.screen.width();

        for y in 1..screen_height {
            for x in 0..screen_width {
                let (c, fg, bg) = try!(self.screen.get(x, y));
                try!(self.screen.set(x, y - 1, c, fg, bg));
            }
        }
        try!(self.clear_row(screen_height-1));
        self.current_x = 0;

        Ok(())
    }

    fn clear_row(&mut self, y: usize) -> Result<()> {
        for x in 0..self.screen.width() {
            try!(self.screen.set(x, y, b' ', self.foreground, self.background));
        }

        Ok(())
    }

}

impl<T: vga::Screen> fmt::Write for Console<T> {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          try!(self.write_byte(byte));
        }
        Ok(())
    }

}

#[derive(Debug)]
pub enum Error {
    Screen(vga::screen::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<vga::screen::Error> for Error {

    fn from(err: vga::screen::Error) -> Self {
        Error::Screen(err)
    }

}

impl From<Error> for fmt::Error {

    fn from(_err: Error) -> Self {
        fmt::Error
    }

}
