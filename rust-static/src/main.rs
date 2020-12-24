// "Hello, world!" Rust no-std version.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use sc::syscall;

fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        syscall!(WRITE, fd, buf as usize, len);
    }
}

fn exit(status: usize) -> ! {
    unsafe {
        syscall!(EXIT, status);
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
