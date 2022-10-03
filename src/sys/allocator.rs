#![cfg(feature = "allocator")]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use alloc::ffi::CString;
use alloc::format;
use crate::ffi::*;


#[global_allocator]
pub static ALLOCATOR: Furi = Furi;

pub struct Furi;

unsafe impl GlobalAlloc for Furi {
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 { aligned_malloc(layout.size(), layout.align()) as *mut u8 }

	#[inline(always)]
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) { aligned_free(ptr as *mut c_void); }

	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { self.alloc(layout) }
}

#[alloc_error_handler]
fn on_oom(layout: Layout) -> ! {
	unsafe {
		furi_thread_yield();
		let message = CString::from_vec_unchecked(format!(
			"OoM: requested {}b, align: {}, free: {}b",
			layout.size(),
			layout.align(),
			memmgr_get_free_heap()
		).into_bytes());
		crate::sys::crash(message.as_ptr() as _)
	}
}
