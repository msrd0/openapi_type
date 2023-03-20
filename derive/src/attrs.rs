use crate::util::ExpectLit;
use proc_macro2::TokenStream;
use syn::{
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	spanned::Spanned as _,
	Attribute, LitStr, Meta, Token
};

pub(super) fn parse_doc_attr(input: &Attribute) -> syn::Result<Option<LitStr>> {
	Ok(match &input.meta {
		Meta::NameValue(kv) => Some(kv.value.clone().expect_str()?),
		_ => None
	})
}

fn unknown(meta: Meta, error_on_unknown: bool) -> syn::Result<()> {
	if error_on_unknown {
		Err(syn::Error::new(
			match meta {
				Meta::Path(path) => path.span(),
				Meta::List(list) => list.span(),
				Meta::NameValue(kv) => kv.path.span()
			},
			"Unexpected token"
		))
	} else {
		Ok(())
	}
}

#[derive(Default)]
pub(super) struct ContainerAttributes {
	pub(super) doc: Vec<String>,
	pub(super) rename: Option<LitStr>,
	pub(super) rename_all: Option<LitStr>,
	pub(super) tag: Option<LitStr>,
	pub(super) content: Option<LitStr>,
	pub(super) untagged: bool,
	pub(super) deny_unknown_fields: bool
}

struct ParseHelper(Punctuated<Meta, Token![,]>);

impl Parse for ParseHelper {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		Ok(Self(Punctuated::parse_terminated(input)?))
	}
}

impl ContainerAttributes {
	pub(super) fn parse_from(&mut self, tokens: TokenStream, error_on_unknown: bool) -> syn::Result<()> {
		let tokens = syn::parse2::<ParseHelper>(tokens)?.0;
		for token in tokens {
			match token {
				Meta::NameValue(kv) if kv.path.is_ident("rename") => {
					self.rename = Some(kv.value.expect_str()?);
				},

				Meta::NameValue(kv) if kv.path.is_ident("rename_all") => {
					self.rename_all = Some(kv.value.expect_str()?);
				},

				Meta::NameValue(kv) if kv.path.is_ident("tag") => {
					self.tag = Some(kv.value.expect_str()?);
				},

				Meta::NameValue(kv) if kv.path.is_ident("content") => {
					self.content = Some(kv.value.expect_str()?);
				},

				Meta::Path(path) if path.is_ident("untagged") => {
					self.untagged = true;
				},

				Meta::Path(path) if path.is_ident("deny_unknown_fields") => {
					self.deny_unknown_fields = true;
				},

				meta => unknown(meta, error_on_unknown)?
			}
		}
		Ok(())
	}
}

#[derive(Default)]
pub(super) struct FieldAttributes {
	/// Use a different name for the spec than in Rust code.
	pub(super) rename: Option<LitStr>,
	/// This field can be skipped during either serialization or deserialization.
	pub(super) nullable: bool,
	/// This field's fields will be flattened into this field.
	pub(super) flatten: bool,
	/// This field will always be skipped during serialization.
	pub(super) skip_serializing: bool,
	/// This field will always be skipped during deserialization.
	pub(super) skip_deserializing: bool
}

impl FieldAttributes {
	pub(super) fn parse_from(&mut self, tokens: TokenStream, error_on_unknown: bool) -> syn::Result<()> {
		let tokens = syn::parse2::<ParseHelper>(tokens)?.0;
		for token in tokens {
			match token {
				Meta::NameValue(kv) if kv.path.is_ident("rename") => {
					self.rename = Some(kv.value.expect_str()?);
				},

				Meta::Path(path) if path.is_ident("default") => {
					self.nullable = true;
				},

				Meta::NameValue(kv) if kv.path.is_ident("default") => {
					self.nullable = true;
				},

				Meta::Path(path) if path.is_ident("flatten") => {
					self.flatten = true;
				},

				Meta::Path(path) if path.is_ident("skip") => {
					self.nullable = true;
					self.skip_serializing = true;
					self.skip_deserializing = true;
				},

				Meta::Path(path) if path.is_ident("skip_serializing") => {
					self.nullable = true;
					self.skip_serializing = true;
				},

				Meta::Path(path) if path.is_ident("skip_deserializing") => {
					self.nullable = true;
					self.skip_deserializing = true;
				},

				Meta::NameValue(kv) if kv.path.is_ident("skip_serializing_if") => {
					self.nullable = true;
				},

				meta => unknown(meta, error_on_unknown)?
			}
		}
		Ok(())
	}
}
