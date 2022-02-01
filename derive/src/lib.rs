#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]
//! This crate defines the macros for `#[derive(OpenapiType)]`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, LitStr, TraitBound, TraitBoundModifier, TypeParamBound};

#[macro_use]
mod util;

mod attrs;
use attrs::*;
mod codegen;
use codegen::*;
mod parser;
use parser::*;

/// The derive macro for [`OpenapiType`].
///
///  [`OpenapiType`]: https://docs.rs/openapi_type/*/openapi_type/trait.OpenapiType.html
#[proc_macro_derive(OpenapiType, attributes(openapi))]
pub fn derive_openapi_type(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input);
	expand_openapi_type(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

fn expand_openapi_type(mut input: DeriveInput) -> syn::Result<TokenStream2> {
	let ident = &input.ident;

	// parse #[serde] and #[openapi] attributes
	let mut attrs = ContainerAttributes::default();
	for attr in &input.attrs {
		if attr.path.is_ident("serde") {
			attrs.parse_from(attr, false)?;
		}
	}
	for attr in &input.attrs {
		if attr.path.is_ident("openapi") {
			attrs.parse_from(attr, true)?;
		}
	}

	// parse #[doc] attributes
	for attr in &input.attrs {
		if attr.path.is_ident("doc") {
			if let Some(lit) = parse_doc_attr(attr)? {
				attrs.doc.push(lit.value());
			}
		}
	}

	// prepare the generics - all impl generics will get `OpenapiType` requirement
	let (impl_generics, ty_generics, where_clause) = {
		let generics = &mut input.generics;
		generics.type_params_mut().for_each(|param| {
			param.colon_token.get_or_insert_with(Default::default);
			param.bounds.push(TypeParamBound::Trait(TraitBound {
				paren_token: None,
				modifier: TraitBoundModifier::None,
				lifetimes: None,
				path: path!(::openapi_type::OpenapiType)
			}));
		});
		generics.split_for_impl()
	};

	// parse the input data
	let parsed = match &input.data {
		Data::Struct(strukt) => parse_struct(ident, strukt, &attrs)?,
		Data::Enum(inum) => parse_enum(ident, inum, &attrs)?,
		Data::Union(union) => parse_union(union)?
	};

	// run the codegen
	let visit_impl = parsed.gen_visit_impl();

	// put the code together
	Ok(quote! {
		#[allow(unused_mut)]
		impl #impl_generics ::openapi_type::OpenapiType for #ident #ty_generics #where_clause {
			fn visit_type<__openapi_type_V>(visitor: &mut __openapi_type_V)
			where
				__openapi_type_V: ::openapi_type::Visitor
			{
				#visit_impl
			}
		}
	})
}
