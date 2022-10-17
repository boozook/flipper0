#![cfg_attr(not(test), no_std)]
#![feature(custom_inner_attributes)]
#![cfg_attr(feature = "allocator", feature(alloc_error_handler))]
#![cfg_attr(feature = "allocator", feature(allocator_api))]
#![cfg_attr(feature = "allocator", feature(alloc_layout_extra))]
#![cfg_attr(feature = "allocator", feature(nonnull_slice_from_raw_parts))]
#![cfg_attr(feature = "panic", feature(core_intrinsics))]

#[macro_use]
extern crate alloc as _;
pub mod alloc;


// re-export proc-macros:
#[allow(unused_imports)]
#[macro_use]
extern crate proc_macros;
pub use proc_macros::*;


pub mod ffi {
	//! Automatically generated low-level bindings.
	//!
	#![doc = core::env!("BINDINGS_METADATA_DOC", "Bindings metadata not found.")]
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

	core::include!(core::env!("BINDINGS", "Bindings not found. Build-script failed."));


	#[no_mangle]
	unsafe fn __exidx_start() -> ! { loop {} }
	#[no_mangle]
	unsafe fn __exidx_end() -> ! { loop {} }
	#[no_mangle]
	unsafe fn __cxa_call_unexpected() -> ! { loop {} }
	#[no_mangle]
	unsafe fn __gnu_Unwind_Find_exidx() -> ! { loop {} }
	#[no_mangle]
	unsafe fn __cxa_begin_cleanup() -> ! { loop {} }
	#[no_mangle]
	unsafe fn __cxa_type_match() -> ! { loop {} }
}


pub mod r#panic;
pub mod error;
pub mod result;
