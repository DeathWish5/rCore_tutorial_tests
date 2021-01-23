#![no_std]
#![no_main]

extern crate user_lib;
use user_lib::exit;

#[allow(unreachable_code)]
#[no_mangle]
pub fn main() -> i32 {
    exit(-1);
    panic!("FAIL: T.T\n");
    0
}