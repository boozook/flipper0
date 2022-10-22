#![cfg(feature = "panic")]
use crate::ffi::*;
use crate::alloc::ffi::CString;
use core::ffi::CStr;
use core::panic::PanicInfo;
use core::str;


/// Global panic handler with minimal allocs as possible
/// and correct aborting instead of `furi_crash` usage
/// that more correct to use aseptically for mem-related reasons.
#[panic_handler]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn panic(panic_info: &PanicInfo<'_>) -> ! {
	let thread_name = unsafe {
		let thread_id = furi_thread_get_current_id();
		if let Some(thread_name) = furi_thread_get_name(thread_id).as_ref() {
			str::from_utf8_unchecked(CStr::from_ptr(thread_name).to_bytes())
		} else {
			"n/a"
		}
	};

	furi_thread_stdout_write(b"[".as_ptr() as _, 1);
	furi_thread_stdout_write(thread_name.as_ptr() as _, thread_name.len());
	furi_thread_stdout_write(b"]: ".as_ptr() as _, 3);

	let (info, len) = {
		let s = format!("{panic_info}\n");
		let len = s.len();
		(CString::from_vec_unchecked(s.into_bytes()), len)
	};
	furi_thread_stdout_write(info.into_raw(), len);
	furi_thread_stdout_write(b"\n\0".as_ptr() as _, 2);


	// TODO: if CLI & logging enabled __only__!
	furi_thread_stdout_flush();

	// crate::sys::crash(message.as_ptr() as _)
	furi_thread_yield();
	crate::process::abort()
}
