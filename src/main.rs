#![no_std] // don't link with standar  library
#![no_main] // disable Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world!";

#[no_mangle] // don't mangle this function (it's the entry point)
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    // panic!("Some panic message");

    loop {}
}

/// This is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
