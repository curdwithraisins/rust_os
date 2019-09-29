#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"HI!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // vga_buffer::print_sth();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("HI!!!").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", and some more: {} and {}", 43434, "qefefqefqefqef").unwrap();

    println!("From println{}", "!");

    panic!("Some panic message");

    loop {}
}

mod vga_buffer;

