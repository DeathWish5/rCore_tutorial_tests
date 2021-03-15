#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

static TESTS: &[&str] = &[
    "ch2_hello_world\0",
    "ch2_power\0",
    "ch2_write1\0",
    "ch3_0_setprio\0",
    "ch3_0_sleep\0",
    "ch3_0_sleep1\0",
    "ch4_mmap0\0",
    "ch4_mmap1\0",
    "ch4_mmap2\0",
    "ch4_mmap3\0",
    "ch4_unmap\0",
    "ch4_unmap2\0",
    "ch5_getpid\0",
    "ch5_spawn0\0",
    "ch5_spawn1\0",
    "ch6_mail0\0",
    "ch6_mail1\0",
    "ch6_mail2\0",
    "ch6_mail3\0",
    "ch7_file0\0",
    "ch7_file1\0",
    "ch7_file2\0",
];

use user_lib::{spawn, waitpid};

#[no_mangle]
pub fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = spawn(*test);
        let mut xstate: i32 = Default::default();
        let wait_pid = waitpid(pid as usize, &mut xstate);
        assert_eq!(pid, wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid, xstate
        );
    }
    println!("ch7 Usertests passed!");
    0
}
