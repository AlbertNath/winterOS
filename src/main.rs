#![no_std] // don't link with standar  library
#![no_main] // disable Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(winter_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use winter_os::println;

#[no_mangle] // don't mangle this function (it's the entry point)
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

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
    winter_os::test_panic_handler(info)
}
