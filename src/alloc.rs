extern crate alloc;
pub use alloc::*;

#[cfg(feature = "allocator")]
use core::alloc::{Allocator, AllocError};
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr::NonNull;
use alloc::ffi::CString;
pub use alloc::format;
use crate::ffi::*;
use crate::os::crash;


#[cfg(feature = "allocator")]
#[cfg_attr(feature = "allocator-global", global_allocator)]
pub static GLOBAL: Furi = Furi;

#[cfg(any(feature = "allocator", feature = "allocator-global"))]
pub struct Furi;

#[cfg(feature = "allocator-global")]
unsafe impl GlobalAlloc for Furi {
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 { aligned_malloc(layout.size(), layout.align()) as *mut u8 }

	#[inline(always)]
	unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) { aligned_free(ptr as *mut c_void); }

	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { self.alloc(layout) }
}


/// Simple `allocator_api` implementation
#[cfg(feature = "allocator")]
unsafe impl Allocator for Furi {
	#[inline]
	fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		match layout.size() {
			0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
			size => {
				let raw_ptr = unsafe { self.alloc(layout) };
				let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
				Ok(NonNull::slice_from_raw_parts(ptr, size))
			},
		}
	}

	#[inline]
	fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
		match layout.size() {
			0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
			size => {
				let raw_ptr = unsafe { self.alloc_zeroed(layout) };
				let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
				Ok(NonNull::slice_from_raw_parts(ptr, size))
			},
		}
	}

	#[inline]
	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) { self.dealloc(ptr.as_ptr(), layout) }
}


/// Out of Memory handler.
#[cfg(feature = "oom-global")]
#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
	unsafe {
		let message = CString::from_vec_unchecked(format!(
			"OoM: requested {}b, align: {}, free: {}b\0",
			layout.size(),
			layout.align(),
			memmgr_get_free_heap()
		).into_bytes());
		crash(message)
	}
}
