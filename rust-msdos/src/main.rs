#![feature(proc_macro_hygiene, asm)]
#![no_main]
#![no_std]

use rusty_asm::rusty_asm;
use core::panic::PanicInfo;
use libc_print;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write(fd: usize, buf: *const u8, len: usize) {
    unsafe {
        rusty_asm! {
            let buf: in("{dx}") = buf;
            let len: in("{cx}") = len;
            let fd: in("{bx}") = fd;
            clobber("ah");
            clobber("al");
            asm("volatile", "intel") {
                "mov ah, 40h
                int 21h"
            }
        }
    }
}

fn exit() -> ! {
    unsafe {
        rusty_asm! { 
            asm("volatile", "intel") {
                "mov ah, 4ch
                 int 21h"
            }
        }
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world!\n";
    let buf = msg.as_ptr();
    let len = msg.len();

    write(1, buf, len);
    exit();
}
