// "Hello, world!" Rust no-std version.

#![no_std]
#![no_main]

use sc::{syscall1, syscall3};
use sc::nr::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        syscall3(WRITE, fd, buf as usize , len);
    }
}

fn exit(status: usize) -> ! {
    unsafe {
        syscall1(EXIT, status);
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world!\n";
    let buf = msg.as_ptr();
    let len = msg.len();

    write(1, buf, len);
    exit(0);
}
