// "Hello, world!" Rust no-std version.

#![no_std]
#![no_main]

use syscall::*;
use syscall::nr::*;
use core::panic::PanicInfo;

const SYSCALL_CLASS_UNIX: usize = 0x2000000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        syscall3(SYSCALL_CLASS_UNIX | WRITE, fd, buf as usize , len);
    }
}

fn exit(status: usize) -> ! {
    unsafe {
        syscall1(SYSCALL_CLASS_UNIX | EXIT, status);
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
