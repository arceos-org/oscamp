#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use axdriver::prelude::{BaseDriverOps, BlockDriverOps, DeviceType};
use std::thread;

const DISK_SIZE: usize = 0x400_0000; // 64M
const BLOCK_SIZE: usize = 0x200; // 512-bytes in default

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Load app from disk ...");

    let mut alldevs = axdriver::init_drivers();
    let mut disk = alldevs.block.take_one().expect("No block dev!");

    assert_eq!(disk.device_type(), DeviceType::Block);
    assert_eq!(disk.device_name(), "virtio-blk");
    assert_eq!(disk.block_size(), BLOCK_SIZE);
    assert_eq!(disk.num_blocks() as usize, DISK_SIZE / BLOCK_SIZE);

    let mut buf = vec![0u8; BLOCK_SIZE];
    assert!(disk.read_block(0, &mut buf).is_ok());

    let worker1 = thread::spawn(move || {
        println!("worker1 checks head:");
        let head = core::str::from_utf8(&buf[3..11]).unwrap_or_else(|e| {
            panic!("bad disk head: {:?}. err {:?}", &buf[0..16], e);
        });
        println!("[{}]", head);
        println!("\nworker1 ok!");
    });

    println!("Wait for workers to exit ...");
    let _ = worker1.join();

    println!("Load app from disk ok!");
}
