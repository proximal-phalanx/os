use crate::serial_print;
use crate::serial_println;
use crate::println;
use crate::exit;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[Success!]");
    }
}

// #[test_case]
// pub fn trivial_assertion() {
//     serial_println!("trivial assertion... ");
//     assert_eq!(1, 1);
//     serial_println!("[Success!]");
// }

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[no_mangle]
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit::exit_qemu(exit::QemuExitCode::Success);
}