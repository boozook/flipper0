#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(min_specialization)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

// re-export proc-macros:
#[allow(unused_imports)]
#[macro_use]
pub extern crate sys;
pub use sys::alloc;
pub use sys::main;


pub mod macros;
pub mod io;
pub mod fs;
pub mod path;
pub mod string;


pub mod ffi {
	pub use sys::ffi::*;
	pub use sys::alloc::ffi::CString;
	pub use core::ffi::CStr;
}


#[const_trait]
trait AsPtr<T> {
	fn as_ptr(&self) -> *mut T;
}


pub mod error {
	#[derive(Debug)]
	pub struct NullPointerError;
	impl core::error::Error for NullPointerError {}
	impl core::fmt::Display for NullPointerError {
		fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "NullPointerError") }
	}
}
