#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use x86_64::instructions::hlt;

use crate::interrupts::init_idt;
mod serial;
mod lang_item;
mod vga_buffer;
mod exit;
mod interrupts;
mod gdt;
#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() }; 
    x86_64::instructions::interrupts::enable();
    #[cfg(test)]
    test_main();
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}