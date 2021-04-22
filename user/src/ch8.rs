pub use super::*;
use syscall::*;

pub const PAGE_SIZE: usize = 4096;
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub fn forktest<F>(func: F)
where
    F: FnOnce(usize),
{
    let n: usize = 200;
    for idx in 0..n {
        let pid = fork();
        if pid == 0 {
            func(idx);
            exit(0);
        }
    }
    let mut exit_code: i32 = 0;
    for _ in 0..n {
        assert!(wait(&mut exit_code) > 0);
        assert_eq!(exit_code, 0);
    }
    assert!(wait(&mut exit_code) < 0);
}

pub fn get_pc() -> usize {
    let mut ra: usize;
    unsafe {
        llvm_asm!("mv $0, ra" : "=r"(ra) ::: "volatile");
    }
    ra
}

pub fn raw_sys_gettime(tx: *const TimeVal, tz: usize) -> isize {
    syscall(SYSCALL_GETTIMEOFDAY, [tx as usize, tz, 0])
}

pub fn raw_sys_fstat(fd: usize, st: *const Stat) -> isize {
    syscall(SYSCALL_FSTAT, [fd, st as usize, 0])
}

pub fn hash(n: usize) -> usize {
    let h: usize = 6364136223846793005usize * n + 1;
    h >> 33
}
