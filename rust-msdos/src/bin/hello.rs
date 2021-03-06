#![no_main]
#![no_std]

use dos::*;

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = "Hello, world!\r\n";
    let buf = msg.as_ptr();
    let len = msg.len() as u16;

    write(1, buf, len);
    abort();
}
