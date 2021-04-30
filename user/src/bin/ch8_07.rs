#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::ch8::hash;
use user_lib::{open, unlink, OpenFlags};

fn file_test0(idx: usize) {
    let mut name: [u8; 20] = [0; 20];
    let mut last: u8 = idx as u8;
    for c in &mut name {
        *c = hash(last.into()) as u8;
        last = *c;
    }
    name[19] = 0;
    let fname = unsafe { core::str::from_utf8_unchecked(&name) };
    open(fname, OpenFlags::CREATE | OpenFlags::WRONLY);
    unlink(fname);
}

const NUM: usize = 65536;

#[no_mangle]
pub fn main() -> i32 {
    for idx in 0..NUM {
        file_test0(idx);
    }
    0
}
