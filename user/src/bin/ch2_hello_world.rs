#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

/// 正确输出：
/// Hello world from user mode program!
/// Test hello_world OK!

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello world from user mode program!\nTest hello_world OK!");
    0
}
