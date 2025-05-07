#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    #[cfg(target_arch = "riscv64")]
    core::arch::asm!(
        "li a7, 8",
        "ecall",
        options(noreturn)
    );
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
        "mov rax, 60",
        "xor rdi, rdi",
        "syscall",
        options(noreturn)
    );
    #[cfg(target_arch = "aarch64")]
    core::arch::asm!(
        "mov x8, 93",
        "mov x0, 0",
        "svc 0",
        options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
