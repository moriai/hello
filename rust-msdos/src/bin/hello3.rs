// Hello using crate heapless 
#![no_main]
#![no_std]

use dos::*;
use heapless::Vec;
use heapless::String;
use heapless::consts::*;

macro_rules! assert {
    ($cond:expr) => { if ($cond) == false { exit(line!() as u8); }};
}

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() {
    {
        let mut v: Vec<u8, U8> = Vec::new();
        v.push('H' as u8).unwrap();
        v.push('e' as u8).unwrap();
        v.push('l' as u8).unwrap();
        v.push('l' as u8).unwrap();
        v.push('o' as u8).unwrap();
        v.push(',' as u8).unwrap();
        v.push(' ' as u8).unwrap();
        let s = String::from_utf8(v).unwrap();
        assert!(s.len() == 7);
        write(1, s.as_ptr(), s.len() as u16);
    }

    {
        let mut v: Vec<u8, U8> = Vec::new();
        v.push('W' as u8).unwrap();
        v.push('o' as u8).unwrap();
        v.push('r' as u8).unwrap();
        v.push('l' as u8).unwrap();
        v.push('d' as u8).unwrap();
        v.push('!' as u8).unwrap();
        v.push('\r' as u8).unwrap();
        v.push('\n' as u8).unwrap();
        let s = String::from_utf8(v).unwrap();
        assert!(s.len() == 8);
        write(1, s.as_ptr(), s.len() as u16);
    }

    assert!(false);
}
