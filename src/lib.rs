#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(ptr_internals)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

mod common;
mod x86;

// #[macro_use]
// mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    use common::vga::Screen;
    use core::fmt::Write;

    let screen = x86::vga::Screen::new();
    let mut console = common::vga::Console::new(screen);
    console.write_fmt(format_args!("Hello World!")).unwrap();

    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
