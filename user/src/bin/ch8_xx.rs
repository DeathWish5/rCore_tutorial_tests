#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::ch8::*;
use user_lib::{open, unlink, OpenFlags};

#[allow(dead_code)]
const SYSCALL_NUM: usize = 20;

#[allow(dead_code)]
const SYSCALL_IDS: [usize; SYSCALL_NUM] = [
    SYSCALL_OPENAT,       //  usize = 56;
    SYSCALL_CLOSE,        //  usize = 57;
    SYSCALL_READ,         //  usize = 63;
    SYSCALL_WRITE,        //  usize = 64;
    SYSCALL_UNLINKAT,     //  usize = 35;
    SYSCALL_LINKAT,       //  usize = 37;
    SYSCALL_FSTAT,        //  usize = 80;
    SYSCALL_EXIT,         //  usize = 93;
    SYSCALL_YIELD,        //  usize = 124;
    SYSCALL_GETTIMEOFDAY, //  usize = 169;
    SYSCALL_GETPID,       //  usize = 172;
    SYSCALL_FORK,         //  usize = 220;
    SYSCALL_EXEC,         //  usize = 221;
    SYSCALL_WAITPID,      //  usize = 260;
    SYSCALL_SET_PRIORITY, //  usize = 140;
    SYSCALL_MUNMAP,       //  usize = 215;
    SYSCALL_MMAP,         //  usize = 222;
    SYSCALL_SPAWN,        //  usize = 400;
    SYSCALL_MAIL_READ,    //  usize = 401;
    SYSCALL_MAIL_WRITE,   //  usize = 402;
];

#[allow(dead_code)]
fn rand_syscall_id() -> usize {
    0
}

#[no_mangle]
pub fn main() -> i32 {
    // TODO: a naive fuzzy
    0
}
