#![no_std]
#![no_main]
#![feature(llvm_asm)]

extern crate user_lib;

/// 由于 rustsbi 的问题，该程序无法正确退出

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        llvm_asm!("sret");
    }
    panic!("FAIL: T.T\n");
}