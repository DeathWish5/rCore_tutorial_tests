#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const SIZE: usize = 10;
const P: u32 = 3;
const STEP: usize = 100000;
const MOD: u32 = 10007;

/// 正确输出：
/// 3^10000=5079
/// 3^20000=8202
/// 3^30000=8824
/// 3^40000=5750
/// 3^50000=3824
/// 3^60000=8516
/// 3^70000=2510
/// 3^80000=9379
/// 3^90000=2621
/// 3^100000=2749
/// Test power OK!

#[no_mangle]
fn main() -> i32 {
    let mut pow = [0u32; SIZE];
    let mut index: usize = 0;
    pow[index] = 1;
    for i in 1..=STEP {
        let last = pow[index];
        index = (index + 1) % SIZE;
        pow[index] = last * P % MOD;
        if i % 10000 == 0 {
            println!("{}^{}={}", P, i, pow[index]);
        }
    }
    println!("Test power OK!");
    0
}
