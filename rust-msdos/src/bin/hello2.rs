#![feature(llvm_asm)]
#![no_main]
#![no_std]

use dos::*;
use core::result::Result;

#[inline(always)]
fn write(fd: u16, buf: *const u8, len: u16) -> Result<u16, u16> {
    unsafe {
        let cnt: u16;
        let stat: u16;
        
        llvm_asm!("
            mov ah, 40h
            int 21h
            pushf
            pop dx"
            : "={ax}"(cnt), "={dx}"(stat)
            : "{dx}"(buf), "{cx}"(len), "{bx}"(fd)
            :
            : "volatile", "intel");
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
