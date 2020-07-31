#![feature(llvm_asm)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn exit(status: usize) -> ! {
    unsafe {
        let status = status as u8;
        llvm_asm!("
            mov ah, 4ch
            int 21h"
            : : "{al}"(status) : : "volatile", "intel");
    }
    loop {}
}

fn idiv(x: i16, y: i8) -> i8 {
    unsafe {
        let r: i8;
        llvm_asm!(
            "idiv cl"
            : "={al}"(r): "{ax}"(x), "{cl}"(y) : : "volatile", "intel");
        r
    }
}

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let out = idiv(27, 3);
    exit(out as usize);
}
