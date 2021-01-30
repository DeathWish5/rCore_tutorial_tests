#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::mmap;

#[no_mangle]
fn main() -> i32 {
    let start: usize = 0x10000000;
    let len: usize = 4096;
    let prot: usize = 3;
    assert_eq!(len as isize, mmap(start, len, prot));
    assert_eq!(mmap(start - len, len + 1, prot), -1);
    assert_eq!(mmap(start + len, len * 2, prot), (len * 2) as isize);
    for i in start..(start + len * 3) {
        let addr: *mut u8 = i as *mut u8;
        unsafe {
            *addr = i as u8;
        }
    }
    for i in start..(start + len * 3) {
        let addr: *mut u8 = i as *mut u8;
        unsafe {
            assert_eq!(*addr, i as u8);
        }
    }
    println!("Test 04_4 OK!");
    0
}