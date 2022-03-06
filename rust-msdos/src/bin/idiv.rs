#![no_main]
#![no_std]

use core::arch::asm;
use dos::*;

#[inline(always)]
fn idiv(x: i16, y: i8) -> i8 {
    unsafe {
        let r: i8;
        asm!(
            "idiv cl",
            in("ax") x,
            in("cl") y,
            lateout("al") r
        );
        r
    }
}

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let out = idiv(27, 3);
    exit(out as u8);
}
