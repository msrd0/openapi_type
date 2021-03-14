use crate::util::ExpectLit;
use syn::{punctuated::Punctuated, spanned::Spanned as _, Attribute, LitStr, Meta, Token};

#[derive(Default)]
pub(super) struct ContainerAttributes {
	pub(super) rename: Option<LitStr>,
	pub(super) rename_all: Option<LitStr>,
	pub(super) tag: Option<LitStr>,
	pub(super) content: Option<LitStr>,
	pub(super) untagged: bool
}

pub(super) fn parse_container_attrs(
	input: &Attribute,
	attrs: &mut ContainerAttributes,
	error_on_unknown: bool
) -> syn::Result<()> {
	let tokens: Punctuated<Meta, Token![,]> = input.parse_args_with(Punctuated::parse_terminated)?;
	for token in tokens {
		match token {
			Meta::NameValue(kv) if kv.path.is_ident("rename") => {
				attrs.rename = Some(kv.lit.expect_str()?);
			},

			Meta::NameValue(kv) if kv.path.is_ident("rename_all") => {
				attrs.rename_all = Some(kv.lit.expect_str()?);
			},

			Meta::NameValue(kv) if kv.path.is_ident("tag") => {
				attrs.tag = Some(kv.lit.expect_str()?);
			},

			Meta::NameValue(kv) if kv.path.is_ident("content") => {
				attrs.content = Some(kv.lit.expect_str()?);
			},

			Meta::Path(path) if path.is_ident("untagged") => {
				attrs.untagged = true;
			},

			Meta::Path(path) if error_on_unknown => return Err(syn::Error::new(path.span(), "Unexpected token")),
			Meta::List(list) if error_on_unknown => return Err(syn::Error::new(list.span(), "Unexpected token")),
			Meta::NameValue(kv) if error_on_unknown => return Err(syn::Error::new(kv.path.span(), "Unexpected token")),
			_ => {}
		}
	}
	Ok(())
}
