#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {{
		use core::fmt::Write;
		core::write!($crate::io::Stdout, "{}", format_args!($($arg)*)).unwrap()
	}};
}

#[macro_export]
macro_rules! println {
	() => {{
		$crate::print!("\n")
	}};

	($($arg:tt)*) => {{
		$crate::print!("{}\n", format_args!($($arg)*));
	}};
}
