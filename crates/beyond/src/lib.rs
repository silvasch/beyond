//! Easily execute rust functions on an external machine instead of locally.
//!
//! Have a look at https://github.com/silvasch/beyond/tree/main/crates/beyond_example
//! for an example on how to use this library.

pub use beyond_derive::Beyond;

mod error;
pub use error::Error;

#[doc(hidden)]
pub mod serde;

pub mod ssh;
