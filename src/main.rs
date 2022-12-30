#![no_std] // don't link with standar  library
#![no_main] // disable Rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle this function (it's the entry point)
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
