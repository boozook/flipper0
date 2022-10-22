#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#[macro_use]
extern crate flipper0_macro;


#[test]
fn exists() {
	let _ = &as_is_unsafe::as_is_unsafe;
	let _ = &as_is_safe::as_is_safe;
	let _ = &no_abi::no_abi;
	let _ = &no_pub::no_pub;
	let _ = &no_no_mangle::no_no_mangle;
	let _ = &no_input::no_input;
	let _ = &no_ret::no_ret;
	let _ = &hole_ret::hole_ret;
	let _ = &ret_result::ret_result;
	let _ = &ret_result_unsafe::ret_result_unsafe;
	let _ = &input_into::input_into;
}

#[test]
fn inout() {
	let p = std::ptr::null_mut();
	unsafe {
		assert_eq!(0, as_is_unsafe::as_is_unsafe(p));
		assert_eq!(0, as_is_safe::as_is_safe(p));
		assert_eq!(0, no_abi::no_abi(p));
		assert_eq!(0, no_pub::no_pub(p));
		assert_eq!(0, no_no_mangle::no_no_mangle(p));
		assert_eq!(0, no_input::no_input(p));
		assert_eq!(0, no_ret::no_ret(p));
		assert_eq!(0, hole_ret::hole_ret(p));
		assert_eq!(0, ret_result::ret_result(p));
		assert_eq!(0, ret_result_unsafe::ret_result_unsafe(p));
		assert_eq!(0, input_into::input_into(p));
	}
}

#[test]
#[should_panic(expected = "oops")]
fn ends_with_err() {
	unsafe {
		ret_error_unsafe::ret_error_unsafe(std::ptr::null_mut());
	}
}


pub mod as_is_unsafe {
	#[main]
	#[no_mangle]
	pub unsafe extern "C" fn as_is_unsafe(_: *mut u8) -> i32 { 0 }
	const EXPORTED: unsafe extern "C" fn(*mut u8) -> i32 = as_is_unsafe;
}

pub mod as_is_safe {
	#[main]
	#[no_mangle]
	pub extern "C" fn as_is_safe(_: *mut u8) -> i32 { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = as_is_safe;
}

pub mod no_abi {
	#[main]
	#[no_mangle]
	pub fn no_abi(_: *mut u8) -> i32 { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = no_abi;
}

pub mod no_pub {
	#[main]
	#[no_mangle]
	fn no_pub(_: *mut u8) -> i32 { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = no_pub;
}

pub mod no_no_mangle {
	#[main]
	fn no_no_mangle(_: *mut u8) -> i32 { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = no_no_mangle;
}

pub mod no_input {
	#[main]
	fn no_input() -> i32 { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = no_input;
}

pub mod no_ret {
	#[main]
	fn no_ret() { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = no_ret;
}

pub mod hole_ret {
	#[main]
	fn hole_ret() -> _ { 0 }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = hole_ret;
}

pub mod ret_result {
	#[main]
	fn ret_result(_: *mut u8) -> Result<(), &'static str> { Ok(()) }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = ret_result;
}

pub mod ret_result_unsafe {
	#[main]
	unsafe fn ret_result_unsafe(_: *mut u8) -> Result<(), &'static str> { Ok(()) }
	const EXPORTED: unsafe extern "C" fn(*mut u8) -> i32 = ret_result_unsafe;
}

pub mod ret_error_unsafe {
	#[main]
	unsafe fn ret_error_unsafe(_: *mut u8) -> Result<(), &'static str> { Err("oops") }
	const EXPORTED: unsafe extern "C" fn(*mut u8) -> i32 = ret_error_unsafe;
}

pub mod input_into {
	#[main]
	fn input_into(_: Foo) -> Result<(), &'static str> { Ok(()) }
	const EXPORTED: extern "C" fn(*mut u8) -> i32 = input_into;

	pub struct Foo;
	impl From<*mut u8> for Foo {
		fn from(_: *mut u8) -> Self { Self }
	}
}
