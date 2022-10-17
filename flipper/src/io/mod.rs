use core::fmt::Write;


pub mod stdout {}


pub fn stdout() -> Stdout { Stdout }
pub fn print(s: &str) { Stdout.write_str(s).unwrap(); }


pub struct Stdout;

impl core::fmt::Write for Stdout {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		use crate::ffi::furi_thread_stdout_write;

		unsafe {
			if furi_thread_stdout_write(s.as_ptr() as _, s.len()) != s.len() {
				return Err(core::fmt::Error);
			}
		}

		Ok(())
	}
}

impl Stdout {
	pub fn flush(&mut self) -> core::fmt::Result {
		use crate::ffi::furi_thread_stdout_flush;

		unsafe {
			if furi_thread_stdout_flush() != 0 {
				return Err(core::fmt::Error);
			}
		}
		Ok(())
	}
}


// TODO: impl Buffers on furi_stream_buffer
