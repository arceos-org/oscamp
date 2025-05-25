#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use core::{mem, str};
use std::os::arceos::modules::axhal::mem::phys_to_virt;

#[cfg(target_arch = "riscv64")]
/// Physical address for pflash#1
const PFLASH_START: usize = 0x2200_0000;
#[cfg(target_arch = "aarch64")]
/// Physical address for pflash#1
const PFLASH_START: usize = 0x0400_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    // Makesure that we can access pflash region.
    let va = phys_to_virt(PFLASH_START.into()).as_usize();
    let ptr = va as *const u32;
    unsafe {
        println!("Try to access dev region [{:#X}], got {:#X}", va, *ptr);
        let magic = mem::transmute::<u32, [u8; 4]>(*ptr);
        println!("Got pflash magic: {}", str::from_utf8(&magic).unwrap());
    }
}
