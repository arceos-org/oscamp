#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::collections::*;

#[cfg(feature = "alloc")]
pub use hashbrown::*;
