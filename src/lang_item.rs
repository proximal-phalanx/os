use core::panic::PanicInfo;
use crate::serial_print;
use crate::serial_println;
use crate::exit;
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;
    println!("{}", info);
    loop {}
}
#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit::exit_qemu(exit::QemuExitCode::Failed);
    loop {}
}