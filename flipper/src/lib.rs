#![cfg_attr(not(test), no_std)]

// re-export proc-macros:
#[allow(unused_imports)]
#[macro_use]
pub extern crate bindings;
pub use bindings::main;
