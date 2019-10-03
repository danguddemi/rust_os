// Don't link the Rust standard library and remove Rust-level entry points
#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle] // Compile the name exactly so that the linker can find the default `_start`
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    loop {}
}


#[panic_handler] // Handles the panic call
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
