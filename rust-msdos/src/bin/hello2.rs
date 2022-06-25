#![no_main]
#![no_std]

use dos::*;
use core::arch::asm;
use core::result::Result;

#[inline(always)]
fn write(fd: u16, buf: *const u8, len: u16) -> Result<u16, u16> {
    unsafe {
        let status: u16;
        let flags: u16;

        asm!(
            "mov ah, 0x40",
            "int 0x21",
            "pushf",
            "pop dx",
            in("dx") buf,
            in("cx") len,
            in("bx") fd,
            out("ax") status,
            lateout("dx") flags
        );
        if flags & 0x01 != 0 {
            Err(status)
        } else {
            Ok(status)
        }
    }
}

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world!\r\n";
    let status = match write(1, msg.as_ptr(), msg.len() as u16) {
        Ok(count) => count as u8,
        Err(code) => !code as u8
    };
    exit(status);
}
