#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::mmap;

const UNUSED_START: usize = 0x10000;
const N: usize = 0x800;
const LEN: usize = 0x10000;

#[no_mangle]
pub fn main() -> i32 {
    let prot = 3usize;
    for i in 0..(N * 2) {
        mmap(UNUSED_START + i * LEN, LEN, prot);
    }
    0
}
