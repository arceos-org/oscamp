#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "riscv64")]
    core::arch::asm!(
        "csrr a1, mhartid",
        "ld a0, 64(zero)",
        "li a7, 8",
        "ecall",
        options(noreturn)
    );
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        // x86_64 没有直接对应 mhartid 的方式，可以用 CPUID 但这里简化处理
        "xor rsi, rsi",          // 清零 rsi (对应 a1)
        "mov rdi, [64]",         // 从地址 64 加载值到 rdi (对应 a0)
        "mov rax, 8",            // 系统调用号
        "syscall",               // 执行系统调用
        options(noreturn)
    );
    #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
        "mrs x1, mpidr_el1",     // 获取处理器 ID 到 x1 (对应 a1)
        "mov x9, #64",           // 将地址 64 放入临时寄存器 x9
        "ldr x0, [x9]",          // 从地址 64 加载值到 x0 (对应 a0)
        "mov x8, #8",            // 系统调用号
        "svc #0",                // 执行系统调用
        options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
