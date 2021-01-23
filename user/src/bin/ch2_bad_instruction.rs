#![no_std]
#![no_main]
#![feature(llvm_asm)]

extern crate user_lib;

#[no_mangle]
pub fn main() -> ! {
    unsafe {
        llvm_asm!("sret");
    }
    panic!("FAIL: T.T\n");
}