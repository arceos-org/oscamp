#![cfg_attr(feature="axstd",no_std)]
#![cfg_attr(feature="axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;
extern crate alloc;

use alloc::string::String;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let s = String::from("Hello, axalloc!");
    println!("Alloc String: \"{}\"", s);

    let mut vec = vec![0, 1, 2];
    vec.push(3);
    println!("Alloc Vec: {:?}", vec);
}
