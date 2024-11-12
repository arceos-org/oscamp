#![allow(dead_code)]

use axerrno::LinuxError;
use axhal::arch::TrapFrame;
use axhal::trap::{register_trap_handler, SYSCALL};

const SYS_EXIT: usize = 93;

#[register_trap_handler(SYSCALL)]
fn handle_syscall(tf: &TrapFrame, syscall_num: usize) -> isize {
    println!("handle_syscall ...");
    let ret = match syscall_num {
        SYS_EXIT => axtask::exit(tf.arg0() as _),
        _ => {
            println!("Unimplemented syscall: {}", syscall_num);
            -LinuxError::ENOSYS.code() as _
        }
    };
    ret
}
