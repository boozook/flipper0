use core::ffi::{c_void, c_char};
use crate::alloc::ffi::CString;
use crate::ffi::furi_thread_yield;
use crate::ffi::__furi_crash;


/// Abort current process (thread). Also crashes system.
///
/// Useful for failures by memmgr reasons.
///
/// Wrapper of [`__furi_crash`].
#[inline(always)]
pub fn crash<S: Into<CString>>(message: S) -> ! {
	let s = message.into().into_raw();
	unsafe {
		furi_thread_yield();
		(*(__furi_crash as *const c_void as *const unsafe extern "C" fn(*const c_char) -> !))(s)
	}
}
