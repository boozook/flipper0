#![feature(let_chains)]
#![feature(box_syntax)]
#![feature(box_patterns)]

use proc_macro::TokenStream as StdTokenStream;
use syn::AttributeArgs;
use syn::Error as SynError;
use syn::ItemFn;
use syn::{parse_macro_input};
use quote::ToTokens;


mod manifest;
mod export;


type Error = Box<dyn std::error::Error>;
type Result<T = (), E = self::Error> = std::result::Result<T, E>;


/**
	Transforms input fn to conforms fap entry-point signature,
	for example look at entry-point of any official app such as [that][].
	Only signature transforms, but not a body.

	[that]: https://github.com/flipperdevices/flipperzero-firmware/blob/01f7a3e5b52fa1842bb3117d7adddf059807c9ef/applications/main/nfc/nfc.c#L255

	What exactly does this macro do:
	1. Read function definition and fix it to satisfy Flipper requirements.
	```
	pub unsafe extern "C" fn init(_: *mut u8) -> i32 { 0 }
	```

	Then if `export-fam` feature is enabled:
	1. Read existing previously generated manifest (fam) or try to create new one,
	1. Write passed function name as entry_point to the manifest.
*/
#[proc_macro_attribute]
pub fn main(args: StdTokenStream, input: StdTokenStream) -> StdTokenStream {
	let args = parse_macro_input!(args as AttributeArgs);
	let item = parse_macro_input!(input as ItemFn);
	let entry_point = item.sig.ident.to_string();
	let entry_point_span = item.sig.ident.span();

	let output = export::main(args, item).unwrap_or_else(SynError::into_compile_error);
	let result = output.to_token_stream().into();

	// export
	#[cfg(feature = "export-fam")]
	match manifest::export_main_to_manifest(&entry_point).map_err(|err| SynError::new(entry_point_span, err)) {
		#[cfg(not(feature = "export-fam-infallible"))]
		Err(err) => return err.to_compile_error().into(),
		#[cfg(feature = "export-fam-infallible")]
		Err(err) => println!("Export `fam` failed: {err}"),
		_ => {},
	}

	result
}
