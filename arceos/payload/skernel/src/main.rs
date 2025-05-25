#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "riscv64")]
    core::arch::asm!(
        "li a7, 8",
        "ecall",
        options(noreturn),
    );
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        "mov rax, 8",  // 将系统调用号 8 放入 rax 寄存器
        "syscall",     // 执行系统调用
        options(noreturn),
    );
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!(
        "mov x8, #8",  // 将系统调用号 8 放入 x8 寄存器
        "svc #0",      // 执行系统调用
        options(noreturn),
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
