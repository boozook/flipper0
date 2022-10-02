#![cfg_attr(not(test), no_std)]
#![feature(custom_inner_attributes)]
#![deny(rustdoc::broken_intra_doc_links)]

pub extern crate alloc;


mod consts;


pub mod ffi {
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

	core::include!(core::env!("BINDINGS"));
}


pub mod furi {
	use core::ffi::*;
	use super::ffi::furi_crash;

	/// Crash system
	#[inline(always)]
	pub fn crash(message: *const c_char) -> ! {
		unsafe { (*(furi_crash as *const c_void as *const unsafe extern "C" fn(*const c_char) -> !))(message) }
	}
}


pub mod panic_impl {
	use super::ffi::*;
	use alloc::format;
	use alloc::ffi::CString;
	use core::ffi::CStr;
	use core::panic::PanicInfo;
	use core::str;


	#[panic_handler]
	pub fn panic(panic_info: &PanicInfo<'_>) -> ! {
		let thread_name = unsafe {
			let thread_id = furi_thread_get_current_id();
			if let Some(thread_name) = furi_thread_get_name(thread_id).as_ref() {
				str::from_utf8_unchecked(CStr::from_ptr(thread_name).to_bytes())
			} else {
				"null"
			}
		};

		let message = format!("[{thread_name}] panic: {panic_info}");


		// TODO: if CLI enabled __only__!
		unsafe {
			furi_thread_stdout_write(message.as_ptr() as _, message.len());
			furi_thread_stdout_flush();
		}


		unsafe {
			let message = CString::from_vec_unchecked(message.into_bytes());
			furi_thread_yield();
			crate::furi::crash(message.as_ptr() as _)
		}
	}
}
