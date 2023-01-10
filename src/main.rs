#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod serial;
mod lang_item;
mod vga_buffer;
mod exit;
#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel!("Hello world!");
    info!("This is a test");
    warn!("This is a drill");
    error!("This is just a drill");
    println!("Repeat, this is just a drill.");
    #[cfg(test)]
    test_main();
    loop {}
}