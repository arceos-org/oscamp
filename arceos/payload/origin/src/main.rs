#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "riscv64")]
    core::arch::asm!(
        "addi sp, sp, -4",
        "sw a0, (sp)",
        "li a7, 93",
        "ecall",
        options(noreturn),
    );
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        "mov rax, 60",
        "xor rdi, rdi",
        "syscall",
        options(noreturn),
    );
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!(
        "str w0, [sp, #-4]!",  // 对应 RISC-V 的栈操作
        "mov x8, #93",         // AArch64 系统调用号存放在 x8
        "svc #0",              // 执行系统调用
        options(noreturn),
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
