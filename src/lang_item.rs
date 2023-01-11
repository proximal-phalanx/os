use core::panic::PanicInfo;
#[cfg(test)]
use crate::serial_print;
#[cfg(test)]
use crate::serial_println;
#[cfg(test)]
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