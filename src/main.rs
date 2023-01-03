#![no_std] // don't link with standar  library
#![no_main] // disable Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

static HELLO: &[u8] = b"Hello world!";

#[no_mangle] // don't mangle this function (it's the entry point)
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // panic!("Some panic message");
    #[cfg(test)]
    test_main();

    loop {}
}

/// This is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("A trivial assertion... ");
    assert_eq!(0,1);
    serial_println!("[ok]");
}
