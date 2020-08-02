#![feature(llvm_asm)]
#![no_std]

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

        llvm_asm!("
            mov ah, 40h
            int 21h
            pushf
            pop dx"
            : "={ax}"(result), "={dx}"(eflags)
            : "{dx}"(buf), "{cx}"(len), "{bx}"(fd)
            :
            : "volatile", "intel");
        (result, eflags)
    }
}

#[inline(always)]
pub fn abort() -> ! {
    unsafe {
        llvm_asm!("
            mov ah, 0h
            int 21h"
            : : : : "volatile", "intel");
    }
    loop {}
}

#[inline(always)]
pub fn exit(status: u8) -> ! {
    unsafe {
        llvm_asm!("
            mov ah, 4ch
            int 21h"
            : : "{al}"(status) : : "volatile", "intel");
    }
    loop {}
}
