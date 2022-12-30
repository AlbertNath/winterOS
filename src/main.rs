#![no_std] // don't link with standar  library
#![no_main] // disable Rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello wolrd!";

#[no_mangle] // don't mangle this function (it's the entry point)
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
