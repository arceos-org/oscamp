#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!("li a7, 8", "ecall", options(noreturn))
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
