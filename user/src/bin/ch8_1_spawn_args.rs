#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate user_lib;

use alloc::vec::Vec;
use user_lib::{spawn, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    let args = vec!["args1\0", "args2\0", "last_one\0"];
    let args_addr: Vec<*const u8> = args.iter().map(|arg| arg.as_ptr()).collect();
    let cpid = spawn("cmdline_args\0", args_addr.as_slice());
    let mut exit_code: i32 = 0;
    assert_eq!(waitpid(cpid as usize, &mut exit_code), cpid);
    assert_eq!(exit_code, 0);
    println!("spawn args test passed!");
    0
}
