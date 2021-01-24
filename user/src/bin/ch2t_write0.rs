#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[macro_use]
extern crate user_lib;
extern crate alloc;
use user_lib::{write, STDOUT};
use alloc::slice;

/*
理想结果：程序不报错，正确退出。
*/

const STACK_SIZE: usize = 0x1000;

unsafe fn r_sp() -> usize {
    let mut sp: usize;
    llvm_asm!("mv $0, sp": "=r"(sp) ::: "volatile");
    sp
}

unsafe fn stack_range() -> (usize, usize) {
    let sp = r_sp();
    let top = (sp + STACK_SIZE - 1) & (!(STACK_SIZE - 1));
    (top - STACK_SIZE, top)
}

#[no_mangle]
pub unsafe fn main() -> i32 {
    assert_eq!(write(STDOUT, slice::from_raw_parts(0x0 as *const _, 10)), -1);
    let (bottom, top) = stack_range();
    assert_eq!(write(STDOUT, slice::from_raw_parts((top - 5) as *const _, 10)), -1);
    assert_eq!(write(STDOUT, slice::from_raw_parts((bottom - 5) as *const _, 10)), -1);
    // TODO: test string located in .data section
    println!("Test write2 OK!");
    0
}