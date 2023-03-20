use proc_macro2::{Ident, Span};
use syn::{spanned::Spanned as _, Expr, Lit, LitStr};

/// Convert [Ident], [String] and [&str] into a [LitStr].
pub(super) trait ToLitStr {
	fn to_lit_str(&self) -> LitStr;
}

impl ToLitStr for Ident {
	fn to_lit_str(&self) -> LitStr {
		LitStr::new(&self.to_string(), self.span())
	}
}

impl ToLitStr for String {
	fn to_lit_str(&self) -> LitStr {
		LitStr::new(self, Span::call_site())
	}
}

impl ToLitStr for &str {
	fn to_lit_str(&self) -> LitStr {
		LitStr::new(self, Span::call_site())
	}
}

/// Convert a [Lit] to one specific literal type.
pub(crate) trait ExpectLit {
	fn expect_str(self) -> syn::Result<LitStr>;
}

impl ExpectLit for Lit {
	fn expect_str(self) -> syn::Result<LitStr> {
		match self {
			Self::Str(str) => Ok(str),
			_ => Err(syn::Error::new(self.span(), "Expected string literal"))
		}
	}
}

impl ExpectLit for Expr {
	fn expect_str(self) -> syn::Result<LitStr> {
		match self {
			Expr::Lit(lit) => lit.lit.expect_str(),
			_ => Err(syn::Error::new(self.span(), "Expected string literal"))
		}
	}
}
