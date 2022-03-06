#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
pub fn write(fd: u16, buf: *const u8, len: u16) -> (u16, u16) {
    unsafe {
        let result: u16;
        let eflags: u16;

        asm!(
            "mov ah, 0x40",
            "int 0x21",
            "pushf",
            "pop dx",
            in("dx") buf,
            in("cx") len,
            in("bx") fd,
            out("ax") result,
            lateout("dx") eflags
        );
        (result, eflags)
    }
}

#[inline(always)]
pub fn abort() -> ! {
    unsafe {
        asm!("int 0x20");
    }
    loop {}
}

#[inline(always)]
pub fn exit(status: u8) -> ! {
    unsafe {
        asm!(
            "mov ah, 0x4c",
            "int 0x21",
            in("al") status
        );
    }
    loop {}
}
