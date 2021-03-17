#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::getpid;

/*
理想结果：得到进程 pid，注意要关注 pid 是否符合内核逻辑，不要单纯以 Test OK! 作为判断。
*/

#[no_mangle]
pub fn main() -> i32 {
    let pid = getpid();
    println!("Test getpid OK! pid = {}", pid);
    0
}
