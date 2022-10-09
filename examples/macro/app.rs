#![crate_type = "staticlib"]
#![feature(error_in_core)]
#![no_main]
#![no_std]


#[macro_use]
extern crate flipper0_sys;

use core::ptr::*;
use core::ffi::*;
use alloc::boxed::Box;
use flipper0_sys::ffi::*;
use core::time::Duration;

extern crate alloc;

type Result = core::result::Result<(), Box<dyn core::error::Error>>;


#[main]
pub unsafe fn main() -> Result {
	let view_port = view_port_alloc();
	view_port_draw_callback_set(view_port, Some(draw_callback), null_mut());

	let gui = furi_record_open(RECORD_GUI.as_ptr() as _) as *mut Gui;
	gui_add_view_port(gui, view_port, GuiLayer::GuiLayerFullscreen);

	furi_delay(Duration::from_secs(1));

	view_port_enabled_set(view_port, false);
	gui_remove_view_port(gui, view_port);
	furi_record_close(RECORD_GUI.as_ptr() as _);
	view_port_free(view_port);

	Ok(())
}


pub unsafe extern "C" fn draw_callback(canvas: *mut Canvas, _context: *mut c_void) {
	static MESSAGE: &[u8] = b"Hello, Flipper!";
	let message: &CStr = &CStr::from_ptr(MESSAGE.as_ptr() as _);
	canvas_draw_str(canvas, 39, 31, message.as_ptr());
}


/// Prevents overflow for cast u128 -> u32 for durations of 1h+
unsafe fn furi_delay(duration: Duration) {
	if duration < Duration::from_secs(3600) {
		furi_delay_us(duration.as_micros() as _)
	} else {
		furi_delay_ms(duration.as_millis() as _)
	}
}
