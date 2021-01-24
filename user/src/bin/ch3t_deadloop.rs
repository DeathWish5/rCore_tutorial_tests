#![no_std]
#![no_main]

extern crate user_lib;

/*
理想结果：执行一段时间之后被正确杀死，不会陷入死循环。
*/

#[no_mangle]
pub fn main() -> ! {
    loop{}
}