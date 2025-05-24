#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use core::{mem, str};
use std::thread;
use std::os::arceos::modules::axhal::mem::phys_to_virt;

#[cfg(target_arch = "riscv64")]
/// Physical address for pflash#1
const PFLASH_START: usize = 0x2200_0000;
#[cfg(target_arch = "aarch64")]
/// Physical address for pflash#1
const PFLASH_START: usize = 0x0400_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Multi-task is starting ...");

    let worker = thread::spawn(move || {
        println!("Spawned-thread ...");

        // Makesure that we can access pflash region.
        let va = phys_to_virt(PFLASH_START.into()).as_usize();
        let ptr = va as *const u32;
        let magic = unsafe {
            mem::transmute::<u32, [u8; 4]>(*ptr)
        };
        if let Ok(s) = str::from_utf8(&magic) {
            println!("Got pflash magic: {s}");
            0
        } else {
            -1
        }
    });

    let ret = worker.join();
    // Makesure that worker has finished its work.
    assert_eq!(ret, Ok(0));

    println!("Multi-task OK!");
}
