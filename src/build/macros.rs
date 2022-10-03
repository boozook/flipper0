#![macro_use]

macro_rules! doc_export {
	(mod $item:ident) => {
		#[cfg(doc)]
		pub mod $item;
		#[cfg(not(doc))]
		pub(crate) mod $item;
	};
	($item:ident::*) => {
		#[cfg(doc)]
		pub use $item::*;
		#[cfg(not(doc))]
		pub(crate) use $item::*;
	};
	($item:path) => {
		#[cfg(doc)]
		pub use $item;
		#[cfg(not(doc))]
		pub(crate) use $item;
	};
}
