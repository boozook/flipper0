#![cfg_attr(not(test), no_std)]

// re-export proc-macros:
#[macro_use]
pub extern crate bindings;
pub use bindings::main;
