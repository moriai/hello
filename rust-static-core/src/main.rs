// "Hello, world!" Rust no-std version.

#![no_std]
#![no_main]

use sc::{syscall1, syscall3};
use sc::nr::*;

use core::panic::PanicInfo;
use core::fmt::Write;

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

struct RawFileDesc {
    fd: usize
}

impl Write for RawFileDesc {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(self.fd, s.as_ptr(), s.len());
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "world";
    let mut fd = RawFileDesc{fd:1};
    write!(&mut fd, "Hello, {}!\n", msg).unwrap();
    exit(0);
}
