#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info) 
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u32)]
// pub enum QemuExitCode {
//     Success = 0x10,
//     Failed = 0x11,
// }

// pub fn exit_qemu(exit_code: QemuExitCode) {
//     use x86_64::instructions::port::Port;

//     unsafe {
//         let mut port = Port::new(0xf4);
//         port.write(exit_code as u32);
//     }
// }

// static HELLO: &[u8] = b"HI!";

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

    #[cfg(test)]
    test_main();
    loop {}
}

// mod vga_buffer;
// mod serial;


// #[cfg(test)]
// fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Runnung {} tests", tests.len());
//     for test in tests {
//         test();
//     }
//     exit_qemu(QemuExitCode::Success);
// }

// #[test_case]
// fn trivial_assertion() {
//     serial_println!("trivial assertion...");
//     assert_eq!(1, 1);
//     serial_println!("ok");
// }