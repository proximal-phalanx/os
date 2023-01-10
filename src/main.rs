#![no_std]
#![no_main]
mod lang_item;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");
    loop {}
}