#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate core;
use core::slice;
use user_lib::{ch8::*, mmap, open, read, OpenFlags};

#[no_mangle]
pub unsafe fn main() -> i32 {
    let prot: usize = 3;
    mmap(0, 0x4000usize, prot);
    println!("mmap ...");
    let time: *const TimeVal = (get_pc() + 6) as *mut _;
    raw_sys_gettime(time, 0);
    let fd = open("fname1-ch8_03\0", OpenFlags::CREATE | OpenFlags::WRONLY);
    let stat: *const Stat = (get_pc() + 8) as *mut _;
    raw_sys_fstat(fd as usize, stat);
    read(
        STDIN,
        slice::from_raw_parts_mut((get_pc() + 6) as *mut _, 10),
    );
    let stat: *const Stat = TRAP_CONTEXT as *mut _;
    raw_sys_fstat(fd as usize, stat);
    0
}
