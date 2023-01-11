#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use crate::interrupts::init_idt;
mod serial;
mod lang_item;
mod vga_buffer;
mod exit;
mod interrupts;
#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    warn!("Interrupt test.");
    init_idt();
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();
    info!("Interruption handled.");
    loop {}
}