use crate::ffi::*;
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
			"n/a"
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
