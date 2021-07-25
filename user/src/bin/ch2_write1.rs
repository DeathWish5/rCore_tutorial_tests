#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{write, STDOUT};
const DATA_STRING: &'static str = "string from data section\n";

/// 正确输出：
/// string from data section
/// strinstring from stack section
/// strin
/// Test write1 OK!

#[no_mangle]
pub fn main() -> i32 {
    assert_eq!(write(1234, DATA_STRING.as_bytes()), -1);
    assert_eq!(
        write(STDOUT, DATA_STRING.as_bytes()),
        DATA_STRING.len() as isize
    );
    assert_eq!(write(STDOUT, &DATA_STRING.as_bytes()[..5]), 5);
    // let stack_string = "string from stack section\n";
    let string = "string from stack section!!\n";
    let stack_string_holder: [u8; 50] = [0; 50];
    let stack_string_u8 = unsafe {
        core::ptr::copy(
            string.as_ptr() as *const u8,
            stack_string_holder.as_ptr() as *mut u8,
            string.len(),
        );
        core::slice::from_raw_parts(stack_string_holder.as_ptr(), string.len())
    };
    let stack_string = core::str::from_utf8(stack_string_u8.as_ref()).unwrap();
    assert_eq!(
        write(STDOUT, stack_string.as_bytes()),
        stack_string.len() as isize
    );
    assert_eq!(write(STDOUT, &stack_string.as_bytes()[..5]), 5);
    println!("\nTest write1 OK!");
    0
}
