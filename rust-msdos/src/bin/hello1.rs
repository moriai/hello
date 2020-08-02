#![no_main]
#![no_std]

use dos::*;

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() /*-> !*/ {
    let msg = "Hello, world!\r\n";
    let (result, eflags) = write(1, msg.as_ptr(), msg.len() as u16);
    let status = if eflags & 0x0001 == 0 {
        result as u8
    } else {
        !result as u8
    };
    exit(status);
}
