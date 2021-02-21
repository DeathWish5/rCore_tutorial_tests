#![no_std]
#![no_main]
#![feature(llvm_asm)]

extern crate user_lib;

/// 由于 rustsbi 的问题，该程序无法正确退出

#[no_mangle]
pub fn main() -> ! {
    let mut sstatus: usize;
    unsafe {
        llvm_asm!("csrr $0, sstatus" : "=r"(sstatus) ::: "volatile");
    }
    panic!("(-_-) I get sstatus:{:x}\n", sstatus);
}