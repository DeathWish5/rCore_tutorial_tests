#![no_std]
#![no_main]

extern crate user_lib;
use user_lib::exit;

const MAGIC: usize = 1234;

/// 正确输出： 不输出 FAIL，以 1234 退出

#[allow(unreachable_code)]
#[no_mangle]
pub fn main() -> i32 {
    exit(MAGIC as i32);
    panic!("FAIL: T.T\n");
    0
}
