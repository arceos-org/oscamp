#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::{println,colored_println};

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    colored_println!("31", "[WithColor]: Hello, Arceos!");
    colored_println!("92", "[WithColor]: Hello, Arceos!");
    colored_println!("93", "[WithColor]: Hello, Arceos!");
    colored_println!("95", "[WithColor]: Hello, Arceos!");
}
