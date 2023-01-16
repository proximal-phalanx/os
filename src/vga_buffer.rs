use spin::Mutex;
use lazy_static::lazy_static;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

const DEFAULT_COLOR_CODE: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::LightGreen as u8));

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: DEFAULT_COLOR_CODE,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 可以是能打印的 ASCII 码字节，也可以是换行符
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // 不包含在上述范围之内的字节
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
}

impl Writer {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl Writer {
    fn change_color(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }

    fn change_to_default_color(&mut self){
        self.change_color(DEFAULT_COLOR_CODE);
    }
}

use core::fmt::{self, Arguments};

use crate::interrupts;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

//DEBUG: Print whatever
// pub fn print_something() {
//     use core::fmt::Write;
//     let mut writer = Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::LightGreen, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };

//     writer.write_byte(b'H');
//     writer.write_string("hello! ");
//     write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
// }
//DEBUG END

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

const KERNEL_COLOR: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::Magenta as u8));
const INFO_COLOR: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::LightCyan as u8));
const WARN_COLOR: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::Yellow as u8));
const ERROR_COLOR: ColorCode = ColorCode((Color::Black as u8) << 4 | (Color::LightRed as u8));

#[macro_export]
macro_rules! kernel {
    ($($arg:tt)*) => ($crate::vga_buffer::_kernel(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::vga_buffer::_info(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::vga_buffer::_warn(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::vga_buffer::_error(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        use core::fmt::Write;
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _kernel(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(||{
        let ref mut w = WRITER.lock();
        w.change_color(KERNEL_COLOR);
        w.write_str("[KERNEL] ").unwrap();
        w.write_fmt(args).unwrap();
        w.change_to_default_color();
        w.new_line();
    });
}

#[doc(hidden)]
pub fn _info(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        let ref mut w = WRITER.lock();
        w.change_color(INFO_COLOR);
        w.write_str("[INFO] ").unwrap();
        w.write_fmt(args).unwrap();
        w.change_to_default_color();
        w.new_line();
    });
}

#[doc(hidden)]
pub fn _warn(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        let ref mut w = WRITER.lock();
        w.change_color(WARN_COLOR);
        w.write_str("[WARN] ").unwrap();
        w.write_fmt(args).unwrap();
        w.change_to_default_color();
        w.new_line();
    });
}

#[doc(hidden)]
pub fn _error(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        let ref mut w = WRITER.lock();
        w.change_color(ERROR_COLOR);
        w.write_str("[ERROR] ").unwrap();
        w.write_fmt(args).unwrap();
        w.change_to_default_color();
        w.new_line();
    });
}