#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate core;
use core::slice;
use user_lib::{ch8, *};

#[no_mangle]
pub unsafe fn main() -> i32 {
    let mut bug: [u8; 200] = [0; 200];
    open("fname0\0", OpenFlags::CREATE | OpenFlags::WRONLY);
    open("fname1\0", OpenFlags::CREATE | OpenFlags::WRONLY);
    println!("GOOD LUCK");
    read(1, &mut bug);
    write(65537, slice::from_raw_parts_mut(993 as *mut _, 233));
    read(
        13513543,
        slice::from_raw_parts_mut(0x500 as *mut _, 777777777777usize),
    );
    close(233);
    close(0);
    close(1);
    close(2);
    println!("[ERROR]I need fuzzy ...");
    open(
        "编程是一件危险的事情\0",
        OpenFlags::CREATE | OpenFlags::WRONLY,
    );
    set_priority(-7);
    set_priority(isize::MAX);
    mail_write(100000, slice::from_raw_parts(0 as *const _, 53153));
    mail_write(133, &bug);
    mail_write(0, slice::from_raw_parts(0x1ff0 as *const _, 53153));
    link("nonono\0", "yesyesyes\0");
    link("fname0\0", "fname1\0");
    link("fname1\0", "fname0\0");
    link("fname0\0", "fname0\0");
    link("\0", "fname1\0");
    let stat: *const Stat = 0 as *const _;
    ch8::raw_sys_fstat(0, stat);
    ch8::raw_sys_fstat(313, stat);
    sys_unlinkat(555, "➑➑➑➑➑➑\0", 1);
    sys_linkat(0, "QAQ\0", 7, "❆❆❆❆❆\0", 0);
    0
}
