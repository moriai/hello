#![feature(llvm_asm)]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        llvm_asm!("
            mov ah, 40h
            int 21h"
            : : "{dx}"(buf), "{cx}"(len), "{bx}"(fd) : "al", "ah" : "volatile", "intel");
    }
}

pub fn exit(status: usize) -> ! {
    unsafe {
        let status = status as u8;
        llvm_asm!("
            mov ah, 4ch
            int 21h"
            : : "{al}"(status) : : "volatile", "intel");
    }
    loop {}
}
