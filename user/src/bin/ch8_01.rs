#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::fork;

const NUM: usize = 10;

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..NUM {
        fork();
    }
    0
}
