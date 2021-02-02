#![no_std]
#![no_main]
#![feature(llvm_asm)]

extern crate user_lib;

/*
理想结果：触发 store 指令异常，并杀死程序。
*/

#[no_mangle]
pub fn main() -> isize {
    unsafe {
        (0x0 as *mut u8).write_volatile(0);
    }
    panic!("FAIL: T.T\n");
}