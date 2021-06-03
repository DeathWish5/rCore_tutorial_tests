#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{ch8::forktest, get_time, sleep};

fn heavy_fork_test() {
    for i in 0..30 {
        forktest(|_idx: usize| {
            let current_time = get_time();
            let sleep_length = current_time * current_time % 1000 + 1000;
            sleep(sleep_length as usize);
        });
        println!("Heavy fork test iteration {} success.", i);
    }
}

#[no_mangle]
pub unsafe fn main() -> i32 {
    heavy_fork_test();
    0
}
