#![cfg_attr(not(test), no_std)]
#![feature(custom_inner_attributes)]
#![cfg_attr(feature = "allocator", feature(alloc_error_handler))]

pub extern crate alloc;

mod build {
	mod macros;
	pub mod consts;
}

#[cfg(doc)]
pub use build::consts;


pub mod ffi {
	//! Automatically generated low-level bindings.
	//!
	#![doc = core::env!("BINDINGS_METADATA_DOC", "Bingings metadata not found.")]
	#![allow(non_upper_case_globals)]
	#![allow(non_camel_case_types)]
	#![allow(non_snake_case)]

	#[cfg(all(test, no_std))]
	use core::prelude::*;
	#[cfg(all(test, no_std))]
	use core::prelude::rust_2021::*;
	#[cfg(all(test, no_std))]
	use core::{assert_eq, assert};
	#[cfg(all(test, no_std))]
	use core::{debug_assert_eq, debug_assert};

	core::include!(core::env!("BINDINGS", "Bingings not found. Build-script faild."));
}


pub mod sys {
	pub mod r#panic;
	pub mod allocator;


	use core::ffi::{c_void, c_char};
	use super::ffi::furi_crash;

	/// Crash system
	#[inline(always)]
	pub fn crash(message: *const c_char) -> ! {
		unsafe { (*(furi_crash as *const c_void as *const unsafe extern "C" fn(*const c_char) -> !))(message) }
	}
}
