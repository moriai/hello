// Hello using crate heapless 
#![no_main]
#![no_std]

use dos::*;
use heapless::Vec;
use heapless::String;
//use heapless::consts::*;

macro_rules! assert {
    ($cond:expr) => { if ($cond) == false { exit(line!() as u8); }};
}

#[link_section=".startup"]
#[no_mangle]
pub extern "C" fn _start() {
    {
        let mut v: Vec<char, 8> = Vec::new();
        v.push('H').unwrap();
        v.push('e').unwrap();
        v.push('l').unwrap();
        v.push('l').unwrap();
        v.push('o').unwrap();
        v.push(',').unwrap();
        v.push(' ').unwrap();
        let s: String<8> = v.iter().collect();
        assert!(s.len() == 7);
        write(1, s.as_ptr(), s.len() as u16);
    }

    {
        let mut v: Vec<char, 8> = Vec::new();
        v.push('W').unwrap();
        v.push('o').unwrap();
        v.push('r').unwrap();
        v.push('l').unwrap();
        v.push('d').unwrap();
        v.push('!').unwrap();
        v.push('\r').unwrap();
        v.push('\n').unwrap();
        let s: String<8> = v.iter().collect();
        assert!(s.len() == 8);
        write(1, s.as_ptr(), s.len() as u16);
    }

    assert!(false);
}
