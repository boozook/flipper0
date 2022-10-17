use core::ptr::NonNull;
use core::ffi::CStr;
use core::ffi::c_char;
use core::hash::Hash;
use core::cmp::Ordering;
use core::borrow::Borrow;
use sys::ffi;
use sys::alloc::ffi::CString;
use crate::error::NullPointerError;
use crate::AsPtr;


pub struct OsString(NonNull<ffi::FuriString>);


impl OsString {
	pub fn new() -> Result<Self, NullPointerError> { unsafe { Self::from_raw(ffi::furi_string_alloc()) } }

	unsafe fn from_raw(ptr: *mut ffi::FuriString) -> Result<Self, NullPointerError> {
		NonNull::new(ptr).ok_or(NullPointerError).map(Self)
	}

	/// Length in `char`s.
	pub fn len(&self) -> usize { unsafe { ffi::furi_string_size(self.as_ptr()) } }

	/// Length in utf8 characters.
	pub fn len_utf8(&self) -> usize { unsafe { ffi::furi_string_utf8_length(self.as_ptr()) } }

	pub fn is_empty(&self) -> bool { unsafe { ffi::furi_string_empty(self.as_ptr()) } }


	pub fn get(&self, index: usize) -> c_char { unsafe { ffi::furi_string_get_char(self.as_ptr(), index) } }
	pub fn set(&self, index: usize, char: c_char) { unsafe { ffi::furi_string_set_char(self.as_ptr(), index, char) } }
}

impl Clone for OsString {
	fn clone(&self) -> Self {
		unsafe {
			let ptr = ffi::furi_string_alloc_set(self.as_ptr());
			Self::from_raw(ptr).expect("NullPointer")
		}
	}
}

impl Drop for OsString {
	fn drop(&mut self) {
		unsafe {
			ffi::furi_string_free(self.as_ptr());
		}
	}
}

impl AsRef<CStr> for OsString {
	fn as_ref(&self) -> &CStr {
		unsafe {
			let ptr = ffi::furi_string_get_cstr(self.as_ptr());
			CStr::from_ptr(ptr)
		}
	}
}

impl OsString {
	pub fn to_c_string(&self) -> Result<CString, NullPointerError> {
		unsafe {
			let ptr = ffi::furi_string_get_cstr(self.as_ptr());
			if ptr.is_null() {
				Err(NullPointerError)
			} else {
				Ok(CString::from_raw(ptr as _))
			}
		}
	}

	pub fn as_c_str(&self) -> &CStr { self.as_ref() }
}


impl TryFrom<CString> for OsString {
	type Error = NullPointerError;

	fn try_from(value: CString) -> Result<Self, Self::Error> {
		unsafe {
			let ptr = ffi::furi_string_alloc_set_str(value.into_raw() as _);
			Self::from_raw(ptr)
		}
	}
}

impl TryFrom<&'_ CStr> for OsString {
	type Error = NullPointerError;

	fn try_from(value: &CStr) -> Result<Self, Self::Error> {
		unsafe {
			let ptr = ffi::furi_string_alloc_set_str(value.as_ptr() as _);
			Self::from_raw(ptr)
		}
	}
}


impl AsPtr<ffi::FuriString> for OsString {
	fn as_ptr(&self) -> *mut ffi::FuriString { self.0.as_ptr() }
}


impl core::fmt::Debug for OsString {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{:?}", self.as_c_str()) }
}


impl Borrow<CStr> for OsString {
	#[inline]
	fn borrow(&self) -> &CStr { self.as_c_str() }
}

impl Hash for OsString {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.as_c_str().hash(state); }
}

impl PartialEq<OsString> for OsString {
	fn eq(&self, other: &Self) -> bool { unsafe { ffi::furi_string_equal(self.as_ptr(), other.as_ptr()) } }
}

impl PartialEq<CString> for OsString {
	fn eq(&self, other: &CString) -> bool { self.eq(other.as_c_str()) }
}

impl PartialEq<CStr> for OsString {
	fn eq(&self, other: &CStr) -> bool { unsafe { ffi::furi_string_equal_str(self.as_ptr(), other.as_ptr() as _) } }
}

impl PartialOrd<OsString> for OsString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match unsafe { ffi::furi_string_cmp(self.as_ptr(), other.as_ptr()) } {
			0 => Some(Ordering::Equal),
			1 => Some(Ordering::Greater),
			-1 => Some(Ordering::Less),
			_ => None,
		}
	}
}

impl PartialOrd<CString> for OsString {
	fn partial_cmp(&self, other: &CString) -> Option<Ordering> { self.partial_cmp(other.as_c_str()) }
}

impl PartialOrd<CStr> for OsString {
	fn partial_cmp(&self, other: &CStr) -> Option<Ordering> {
		match unsafe { ffi::furi_string_cmp_str(self.as_ptr(), other.as_ptr() as _) } {
			0 => Some(Ordering::Equal),
			1 => Some(Ordering::Greater),
			-1 => Some(Ordering::Less),
			_ => None,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::ffi::CString;

	#[test]
	#[cfg(target = "thumbv7em-none-eabihf")]
	unsafe fn cmp_os_string() {
		let a = OsString::try_from(CStr::from_ptr(b"a\0".as_ptr() as _)).unwrap();
		let b = OsString::try_from(CStr::from_ptr(b"b\0".as_ptr() as _)).unwrap();
		let result = a.partial_cmp(&b);

		use alloc::string::String;
		let a = String::from("a");
		let b = String::from("b");
		let expected = a.partial_cmp(&b);

		assert_eq!(result, expected);
	}
}
