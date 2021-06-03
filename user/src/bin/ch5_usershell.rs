#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const DL: u8 = 0x7fu8;
const BS: u8 = 0x08u8;

use alloc::string::String;
use user_lib::console::{flush, getchar};
use user_lib::{spawn, waitpid, yield_};

/// 不是测例，方便本地测试

#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut line: String = String::new();
    print!(">> ");
    flush();
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0');
                    let cpid = spawn(line.as_str());
                    if cpid < 0 {
                        println!("invalid file name");
                    } else {
                        let mut xstate: i32 = 0;
                        let mut exit_pid: isize;
                        loop {
                            exit_pid = waitpid(cpid as usize, &mut xstate);
                            if exit_pid == -1 {
                                yield_();
                            } else {
                                assert_eq!(cpid, exit_pid);
                                println!("Shell: Process {} exited with code {}", cpid, xstate);
                                break;
                            }
                        }
                    }
                }
                line.clear();
                print!(">> ");
                flush();
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    flush();
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                flush();
                line.push(c as char);
            }
        }
    }
}
