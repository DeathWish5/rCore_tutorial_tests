#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{
    open,
    close,
    fstat,
    OpenFlags,
    Stat,
    StatMode,
};

#[no_mangle]
pub fn main() -> i32 {
    let fname = "fname1\0";
    let fd = open(fname, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let stat: Stat = Stat::new();
    let ret = fstat(fd, &stat);
    assert_eq!(ret, 0);
    assert_eq!(stat.mode, StatMode::FILE);
    assert_eq!(stat.nlink, 1);
    close(fd);
    println!("TEST fstat OK!");
    0
}