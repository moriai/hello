#![feature(llvm_asm)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        llvm_asm!("
            mov ah, 40h
            int 21h"
            : : "{dx}"(buf), "{cx}"(len), "{bx}"(fd) : "al", "ah" : "volatile", "intel");
    }
}

fn exit(status: usize) -> ! {
    unsafe {
        let status = status as u8;
        llvm_asm!("
            mov ah, 4ch
            int 21h"
            : : "al"(status) : : "volatile", "intel");
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world!\r\n";
    let buf = msg.as_ptr();
    let len = msg.len();

    write(1, buf, len);
    exit(0);
}
