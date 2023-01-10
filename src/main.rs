#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod serial;
mod lang_item;
mod vga_buffer;
mod exit;
mod test;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[Kernel] Hello world!");
    #[cfg(test)]
    test_main();
    loop {}
}