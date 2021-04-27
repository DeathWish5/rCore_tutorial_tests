#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{
    ch8::{forktest, hash},
    mmap,
};

const START: usize = 0x10000;
const LEN: usize = 0x10000;

#[no_mangle]
pub unsafe fn main() -> i32 {
    let prot: usize = 3;
    mmap(START, LEN, prot);
    println!("mmap ...");
    forktest(|idx: usize| {
        let addr: *mut u8 = (START + (hash(idx) % LEN)) as *mut _;
        *addr = 44;
        let addr: *mut u8 = (START + (hash(idx * 65536 + 1) % LEN)) as *mut _;
        *addr;
    });
    0
}
