// "Hello, world!" Rust no-std version.

#![no_std]
#![no_main]

use syscall::*;
use syscall::nr::*;
use core::panic::PanicInfo;
use core::fmt::Write;

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
