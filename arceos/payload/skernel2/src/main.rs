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
        "mov rax, qword ptr [0x40]",  // 从地址0x40(64)加载数据 -> rax
        "cpuid",                       // 获取CPU信息到rbx
        "shr rbx, 24",                // 提取APIC ID (核心ID)
        "mov rdi, rax",               // 参数1：加载的数据
        "mov rsi, rbx",               // 参数2：核心ID
        "mov rax, 8",                 // 设置系统调用号
        "syscall",                    // 执行系统调用
        options(noreturn)
    );
    #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
        "mrs x1, mpidr_el1",         // 读取核心ID寄存器
        "and x1, x1, #0xff",         // 提取当前核心ID
        "ldr x0, [xzr, #64]",        // 从地址64加载数据 -> x0
        "mov x8, #8",                // 设置系统调用号
        "svc #0",                    // 执行系统调用
        options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
