use crate::util::ExpectLit;
use syn::{punctuated::Punctuated, spanned::Spanned as _, Attribute, LitStr, Meta, Token};

pub(super) fn parse_doc_attr(input: &Attribute) -> syn::Result<Option<LitStr>> {
	input.parse_meta().and_then(|meta| {
		Ok(match meta {
			Meta::NameValue(kv) => Some(kv.lit.expect_str()?),
			_ => None
		})
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
	pub(super) rename: Option<LitStr>,
	pub(super) rename_all: Option<LitStr>,
	pub(super) tag: Option<LitStr>,
	pub(super) content: Option<LitStr>,
	pub(super) untagged: bool
}

impl ContainerAttributes {
	pub(super) fn parse_from(&mut self, input: &Attribute, error_on_unknown: bool) -> syn::Result<()> {
		let tokens: Punctuated<Meta, Token![,]> = input.parse_args_with(Punctuated::parse_terminated)?;
		for token in tokens {
			match token {
				Meta::NameValue(kv) if kv.path.is_ident("rename") => {
					self.rename = Some(kv.lit.expect_str()?);
				},

				Meta::NameValue(kv) if kv.path.is_ident("rename_all") => {
					self.rename_all = Some(kv.lit.expect_str()?);
				},

				Meta::NameValue(kv) if kv.path.is_ident("tag") => {
					self.tag = Some(kv.lit.expect_str()?);
				},

				Meta::NameValue(kv) if kv.path.is_ident("content") => {
					self.content = Some(kv.lit.expect_str()?);
				},

				Meta::Path(path) if path.is_ident("untagged") => {
					self.untagged = true;
				},

				meta => unknown(meta, error_on_unknown)?
			}
		}
		Ok(())
	}
}
