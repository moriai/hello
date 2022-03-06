#![no_main]
#![no_std]

use dos::*;
use core::arch::asm;
use core::result::Result;

#[inline(always)]
fn write(fd: u16, buf: *const u8, len: u16) -> Result<u16, u16> {
    unsafe {
        let cnt: u16;
        let stat: u16;

        asm!(
            "mov ah, 0x40",
            "int 0x21",
            "pushf",
            "pop dx",
            in("dx") buf,
            in("cx") len,
            in("bx") fd,
            out("ax") cnt,
            lateout("dx") stat
        );
        if stat & 0x01 != 0 {
            Err(cnt)
        } else {
            Ok(cnt)
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
