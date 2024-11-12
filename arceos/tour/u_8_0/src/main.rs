#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::fs::File;
use std::io::{self, prelude::*};
use std::thread;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Load app from fat-fs ...");

    let mut buf = [0u8; 64];
    if let Err(e) = load_app("/sbin/origin.bin", &mut buf) {
        panic!("Cannot load app! {:?}", e);
    }

    let worker1 = thread::spawn(move || {
        println!("worker1 checks code: ");
        for i in 0..8 {
            print!("{:#x} ", buf[i]);
        }
        println!("\nworker1 ok!");
    });

    println!("Wait for workers to exit ...");
    let _ = worker1.join();

    println!("Load app from disk ok!");
}

fn load_app(fname: &str, buf: &mut [u8]) -> io::Result<usize> {
    println!("fname: {}", fname);
    let mut file = File::open(fname)?;
    let n = file.read(buf)?;
    Ok(n)
}
