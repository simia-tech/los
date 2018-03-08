
use core::fmt;
use super::super::vga;

pub struct Console<T: vga::Screen> {
    screen: T,

    column_position: usize,
    foreground: vga::Color,
    background: vga::Color,
}

impl<T: vga::Screen> Console<T> {

    pub fn new(screen: T) -> Self {
        Console {
            screen: screen,
            column_position: 0,
            foreground: vga::Color::Green,
            background: vga::Color::Black,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                let screen_height = self.screen.height();
                let screen_width = self.screen.width();

                if self.column_position >= screen_width {
                    self.new_line();
                }

                let y = screen_height - 1;
                let x = self.column_position;

                self.screen.set(x, y, byte, self.foreground, self.background);
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        let screen_height = self.screen.height();
        let screen_width = self.screen.width();

        for y in 1..screen_height {
            for x in 0..screen_width {
                let (c, fg, bg) = self.screen.get(x, y);
                self.screen.set(x, y - 1, c, fg, bg);
            }
        }
        self.clear_row(screen_height-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, y: usize) {
        for x in 0..self.screen.width() {
            self.screen.set(x, y, b' ', self.foreground, self.background);
        }
    }

}

impl<T: vga::Screen> fmt::Write for Console<T> {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }

}
