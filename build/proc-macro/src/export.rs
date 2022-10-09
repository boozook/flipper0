use proc_macro2::*;
use quote::ToTokens;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::*;


pub fn main(_args: AttributeArgs, item: ItemFn) -> Result<TokenStream> {
	// first of all check return type and maybe wrap to c-abi function
	let mut item = match add_return_ty_or_wrap(item.clone())? {
		WrapResult::Same(item) => item,
		WrapResult::Wrapper(wrapper) => {
			return Ok(wrapper.to_token_stream());
		},
	};

	// override visibility
	item.vis = Visibility::Public(VisPublic { pub_token: syn::token::Pub(item.vis.span()) });

	// add #[no_mangle]
	add_no_mangle(&mut item);
	// add extern "C"
	add_c_call_abi(&mut item);
	// parameter `_: *mut u8`
	add_parameter(&mut item);

	// check generics parameters
	if !item.sig.generics.params.is_empty() {
		return Err(Error::new(
			item.sig.generics.params.first().unwrap().span(),
			"Generic parameters are not supported.",
		));
	}

	Ok(item.to_token_stream())
}


fn add_no_mangle(f: &mut ItemFn) {
	if f.attrs.is_empty() ||
	   f.attrs
	    .iter()
	    .find(|attr| {
		    attr.path
		        .get_ident()
		        .filter(|ident| &ident.to_string() == "no_mangle")
		        .is_some()
	    })
	    .is_none()
	{
		f.attrs.push(parse_quote! { #[no_mangle] });
	}
}

fn remove_no_mangle(f: &mut ItemFn) {
	let existing = f.attrs
	                .iter()
	                .enumerate()
	                .find(|(_, attr)| attr.path.to_token_stream().to_string() == "no_mangle");
	if let Some((i, _)) = existing {
		f.attrs.remove(i);
	}
}

fn add_c_call_abi(f: &mut ItemFn) {
	if f.sig.abi.is_none() {
		f.sig.abi = Some(parse_quote! { extern });
	}
}

fn add_parameter(f: &mut ItemFn) {
	if f.sig.inputs.is_empty() {
		f.sig.inputs.push(parse_quote! { _: *mut u8 })
	}
}


fn add_return_ty_or_wrap(mut f: ItemFn) -> Result<WrapResult> {
	use syn::token::RArrow;
	use syn::Type::Infer;
	use syn::Type::Never;
	use syn::Type::Path;
	use ReturnType::Type as RetType;
	use ReturnType::Default as RetDefault;

	let default = ReturnType::Type(RArrow::default(), parse_quote! { i32 });
	let mut wrapper = None;
	match f.sig.output {
		RetDefault => f.sig.output = default,
		RetType(_, box Infer(_)) => f.sig.output = default,
		RetType(_, box Path(ref tp)) if result_ty_i32(&tp) => {},
		RetType(_, box Path(ref tp)) => {
			let info = get_result_ty_info(tp)?;
			wrapper = Some(wrap_main_ret_result(f.clone(), info)?);
		},
		RetType(_, box Never(ty)) => {
			// TODO: it should be supported.
			return Err(err_return_type_not_supported(&ty));
		},
		RetType(_, ty) => return Err(err_return_type_not_supported(&ty)),
	}

	if let Some(wrapped) = wrapper {
		Ok(wrapped)
	} else {
		Ok(WrapResult::Same(f))
	}
}


enum WrapResult {
	/// Same function as was before
	Same(ItemFn),
	Wrapper(ItemFn),
}


fn wrap_main_ret_result(mut f: ItemFn, ret: ResultTypeInfo) -> Result<WrapResult> {
	let ident = f.sig.ident.clone();
	let unsafety = f.sig.unsafety.clone();

	// TODO: should it be print-log or panic?
	let fail = quote_spanned!( ret.span => panic!);

	let ok = quote_spanned!( ret.span => Ok(_) => 0);
	let err = quote_spanned!( ret.span => Err(err) => { #fail("{}", err) });

	f.sig.abi = None;
	remove_no_mangle(&mut f);
	f.vis = Visibility::Inherited;

	let (input, call) = match f.sig.inputs.len() {
		0 => (quote_spanned! { f.sig.inputs.span() => (_: *mut u8) }, quote_spanned! { f.sig.inputs.span() => () }),
		_ => (quote_spanned! { f.sig.inputs.span() => (args: *mut u8) }, quote_spanned! { f.sig.inputs.span() => (args.into()) }),
	};

	let wrapper = quote_spanned! { f.span() =>
		#[no_mangle]
		pub #unsafety extern "C" fn #ident #input -> i32 {
			#f
			match #ident #call { #ok, #err, }
		}
	};

	Ok(WrapResult::Wrapper(parse_quote::parse(wrapper)))
}


#[inline]
fn err_return_type_not_supported<T: Spanned + ToTokens>(ty: &T) -> Error {
	Error::new(ty.span(), format!("Return type `{}` is not supported.", ty.to_token_stream()))
}


struct ResultTypeInfo {
	span: Span,
}


impl From<PathSegment> for ResultTypeInfo {
	fn from(seg: PathSegment) -> Self { (&seg).into() }
}

impl From<&'_ PathSegment> for ResultTypeInfo {
	fn from(seg: &'_ PathSegment) -> Self { Self { span: seg.span() } }
}


fn result_ty_i32(tp: &TypePath) -> bool { tp.to_token_stream().to_string() == "i32" }


/// Info for only supported type, otherwise return:
/// - `Ok` if it's a supported `Result`
/// - `Err` it it's not supported result or not a `Result`.
fn get_result_ty_info(tp: &TypePath) -> Result<ResultTypeInfo> {
	let is_result = match tp.path.segments.len() {
		1 => path_segments_to_string(&parse_quote! { Result }) == path_segments_to_string(&tp.path),
		2 => path_segments_to_string(&parse_quote! { result::Result }) == path_segments_to_string(&tp.path),
		3 => path_segments_to_string(&parse_quote! { core::result::Result }) == path_segments_to_string(&tp.path),
		_ => false,
	};

	if is_result {
		let info: ResultTypeInfo = if let Some(result) = tp.path.segments.last() {
			// check generics
			match &result.arguments {
				// OK, save span and link that span to .map(i32::from(result)) or something like that in the wrapper
				PathArguments::None => ResultTypeInfo::from(result),
				PathArguments::AngleBracketed(AngleBracketedGenericArguments { .. }) => ResultTypeInfo::from(result),
				PathArguments::Parenthesized(_) => return Err(err_return_type_not_supported(&result.arguments)),
			}
		} else {
			return Err(err_return_type_not_supported(&tp));
		};

		Ok(info)
	} else {
		Err(err_return_type_not_supported(&tp))
	}
}


fn path_segments_to_string(path: &Path) -> String {
	path.segments
	    .iter()
	    .map(|seg| seg.ident.to_string())
	    .collect::<Vec<_>>()
	    .join("::")
}


#[cfg(test)]
mod tests {
	use super::*;

	// Yeah, partially there's tests of `syn` functionality,
	// but that prevents troubleshooting if somewhere something broken or improved.


	#[test]
	fn test_add_no_mangle() {
		let mut has: ItemFn = parse_quote! { #[no_mangle] fn foo() {} };
		let mut no: ItemFn = parse_quote! { fn foo() {} };

		add_no_mangle(&mut has);
		add_no_mangle(&mut no);

		assert_eq!(1, has.attrs.len());
		assert_eq!(1, no.attrs.len());

		let expected = Some("no_mangle".to_string());
		assert_eq!(expected, has.attrs.get(0).map(|a| a.path.to_token_stream().to_string()));
		assert_eq!(expected, no.attrs.get(0).map(|a| a.path.to_token_stream().to_string()));
	}

	#[test]
	fn test_remove_no_mangle() {
		let mut has: ItemFn = parse_quote! { #[no_mangle] fn foo() {} };
		let mut no: ItemFn = parse_quote! { fn foo() {} };

		remove_no_mangle(&mut has);
		remove_no_mangle(&mut no);

		assert_eq!(0, has.attrs.len());
		assert_eq!(0, no.attrs.len());
	}

	#[test]
	fn test_add_c_call_abi() {
		let mut has_a: ItemFn = parse_quote! { extern "C" fn foo() {} };
		let mut has_b: ItemFn = parse_quote! { extern fn foo() {} };
		let mut no: ItemFn = parse_quote! { fn foo() {} };

		add_c_call_abi(&mut has_a);
		add_c_call_abi(&mut has_b);
		add_c_call_abi(&mut no);

		assert!(has_b.sig.abi.is_some());
		assert!(no.sig.abi.is_some());

		assert_eq!(Some(parse_quote! { extern "C" }), has_a.sig.abi);
		assert_eq!(Some(parse_quote! { extern }), has_b.sig.abi);
		assert_eq!(Some(parse_quote! { extern }), no.sig.abi);
	}

	#[test]
	fn test_add_parameter() {
		let mut has: ItemFn = parse_quote! { fn foo(a: A) {} };
		let mut no: ItemFn = parse_quote! { fn foo() {} };

		add_parameter(&mut has);
		add_parameter(&mut no);

		assert_eq!(1, has.sig.inputs.len());
		assert_eq!(1, no.sig.inputs.len());

		assert_eq!(has.sig.inputs[0], parse_quote! { a: A });
		assert_eq!(no.sig.inputs[0], parse_quote! { _: *mut u8 });
	}

	#[test]
	fn test_result_ty_i32() {
		assert!(result_ty_i32(&parse_quote!(i32)));
		assert!(!result_ty_i32(&parse_quote!(u32)));
	}

	#[test]
	fn test_get_result_ty_info() {
		let tp_res_a: TypePath = parse_quote! { Result<()> };
		let tp_res_b: TypePath = parse_quote! { core::result::Result };
		let tp_foo: TypePath = parse_quote! { Foo };

		assert!(get_result_ty_info(&tp_res_a).is_ok());
		assert!(get_result_ty_info(&tp_res_b).is_ok());
		assert!(get_result_ty_info(&tp_foo).is_err());
	}

	#[test]
	fn test_path_segments_to_string() {
		assert_eq!(path_segments_to_string(&parse_quote! { Foo<()> }), "Foo");
		assert_eq!(path_segments_to_string(&parse_quote! { ::foo::Bar }), "foo::Bar");
	}
}
