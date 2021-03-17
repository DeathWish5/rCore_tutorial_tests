#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::console::getchar;

const N: usize = 10;

/*
测试 sys_read()，目前只能从 stdin 读取。
程序行为：接受 N 个键盘输入并最终一齐输出（注意没有输入时不会显示），如果一致就算正确。不要单纯以 Test getchar passed! 作为判断。
*/

#[no_mangle]
pub fn main() -> i32 {
    println!("please enter {} letters.", N);
    let mut line = [0u8; N];
    for idx in 0..N {
        let c = getchar();
        line[idx] = c;
    }
    println!("{} letters entered", N);
    for idx in 0..N {
        print!("{}", line[idx]);
    }
    println!("\n Test getchar passed!");
    0
}
