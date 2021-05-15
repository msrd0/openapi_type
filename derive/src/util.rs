use proc_macro2::{Ident, Span};
use syn::{Lit, LitStr};

/// Convert any literal path into a [syn::Path].
macro_rules! path {
	(:: $($segment:ident)::*) => {
		path!(@private Some(Default::default()), $($segment),*)
	};
	($($segment:ident)::*) => {
		path!(@private None, $($segment),*)
	};
	(@private $leading_colon:expr, $($segment:ident),*) => {
		{
			#[allow(unused_mut)]
			let mut segments: ::syn::punctuated::Punctuated<::syn::PathSegment, _> = Default::default();
			$(
				segments.push(::syn::PathSegment {
					ident: ::proc_macro2::Ident::new(stringify!($segment), ::proc_macro2::Span::call_site()),
					arguments: Default::default()
				});
			)*
			::syn::Path {
				leading_colon: $leading_colon,
				segments
			}
		}
	};
}

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
		LitStr::new(&self, Span::call_site())
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
