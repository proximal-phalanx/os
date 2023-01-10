/// 这个函数将在 panic 时被调用

// suppress error
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;
    println!("{}", info);
    loop {}
}