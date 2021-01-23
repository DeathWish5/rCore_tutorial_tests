#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{write, STDOUT};
const DATA_STRING:&'static str = "string from data section";

#[no_mangle]
pub fn main() -> i32 {
    assert_eq!(write(STDOUT, DATA_STRING.as_bytes()), DATA_STRING.len() as isize);
    assert_eq!(write(STDOUT, &DATA_STRING.as_bytes()[..5]), 5);
    let stack_string = "string from stack section";
    assert_eq!(write(STDOUT, stack_string.as_bytes()), stack_string.len() as isize);
    assert_eq!(write(STDOUT, &stack_string.as_bytes()[..5]), 5);
    println!("Test write1 OK!");
    0
}