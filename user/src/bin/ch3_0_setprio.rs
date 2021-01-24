#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{set_priority};

/*
理想结果：进程正确退出。
*/

#[no_mangle]
pub fn main() -> i32 {
    assert_eq!(set_priority(10), 10);
    assert_eq!(set_priority(isize::MAX), isize::MAX);
    assert_eq!(set_priority(0), -1);
    assert_eq!(set_priority(1), -1);
    assert_eq!(set_priority(-10), -1);
    println!("TEST set_priority OK!");
    0
}