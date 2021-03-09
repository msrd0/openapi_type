use crate::util::{ExpectLit, ToLitStr};
use proc_macro2::Span;
use syn::{
	punctuated::Punctuated, spanned::Spanned as _, Attribute, DataEnum, DataStruct, DataUnion, Fields, FieldsNamed, LitStr,
	Meta, Token, Type
};

pub(super) enum ParseDataType {
	Type(Type),
	Inline(ParseData)
}

#[allow(dead_code)]
pub(super) enum ParseData {
	Struct(Vec<(LitStr, ParseDataType)>),
	Enum(Vec<LitStr>),
	Alternatives(Vec<ParseData>),
	Unit
}

fn parse_named_fields(named_fields: &FieldsNamed) -> syn::Result<ParseData> {
	let mut fields: Vec<(LitStr, ParseDataType)> = Vec::new();
	for f in &named_fields.named {
		let ident = f
			.ident
			.as_ref()
			.ok_or_else(|| syn::Error::new(f.span(), "#[derive(OpenapiType)] does not support fields without an ident"))?;
		let name = ident.to_lit_str();
		let ty = f.ty.to_owned();
		fields.push((name, ParseDataType::Type(ty)));
	}
	Ok(ParseData::Struct(fields))
}

pub(super) fn parse_struct(strukt: &DataStruct) -> syn::Result<ParseData> {
	match &strukt.fields {
		Fields::Named(named_fields) => parse_named_fields(named_fields),
		Fields::Unnamed(unnamed_fields) => {
			return Err(syn::Error::new(
				unnamed_fields.span(),
				"#[derive(OpenapiType)] does not support tuple structs"
			))
		},
		Fields::Unit => Ok(ParseData::Unit)
	}
}

pub(super) fn parse_enum(inum: &DataEnum, attrs: &ContainerAttributes) -> syn::Result<ParseData> {
	let mut strings: Vec<LitStr> = Vec::new();
	let mut types: Vec<(LitStr, ParseData)> = Vec::new();

	for v in &inum.variants {
		let name = v.ident.to_lit_str();
		match &v.fields {
			Fields::Named(named_fields) => {
				types.push((name, parse_named_fields(named_fields)?));
			},
			Fields::Unnamed(unnamed_fields) => {
				return Err(syn::Error::new(
					unnamed_fields.span(),
					"#[derive(OpenapiType)] does not support tuple variants"
				))
			},
			Fields::Unit => strings.push(name)
		}
	}

	let data_strings = if strings.is_empty() {
		None
	} else {
		match (&attrs.tag, &attrs.content, attrs.untagged) {
			// externally tagged (default)
			(None, None, false) => Some(ParseData::Enum(strings)),
			// internally tagged or adjacently tagged
			(Some(tag), _, false) => Some(ParseData::Struct(vec![(
				tag.clone(),
				ParseDataType::Inline(ParseData::Enum(strings))
			)])),
			// untagged
			(None, None, true) => Some(ParseData::Unit),
			// unknown
			_ => return Err(syn::Error::new(Span::call_site(), "Unknown enum representation"))
		}
	};

	let data_types =
		if types.is_empty() {
			None
		} else {
			Some(ParseData::Alternatives(
				types
					.into_iter()
					.map(|(name, mut data)| {
						Ok(match (&attrs.tag, &attrs.content, attrs.untagged) {
							// externally tagged (default)
							(None, None, false) => ParseData::Struct(vec![(name, ParseDataType::Inline(data))]),
							// internally tagged
							(Some(tag), None, false) => {
								match &mut data {
									ParseData::Struct(fields) => {
										fields.push((tag.clone(), ParseDataType::Inline(ParseData::Enum(vec![name]))))
									},
									_ => return Err(syn::Error::new(
										tag.span(),
										"#[derive(OpenapiType)] does not support tuple variants on internally tagged enums"
									))
								};
								data
							},
							// adjacently tagged
							(Some(tag), Some(content), false) => ParseData::Struct(vec![
								(tag.clone(), ParseDataType::Inline(ParseData::Enum(vec![name]))),
								(content.clone(), ParseDataType::Inline(data)),
							]),
							// untagged
							(None, None, true) => data,
							// unknown
							_ => return Err(syn::Error::new(Span::call_site(), "Unknown enum representation"))
						})
					})
					.collect::<syn::Result<Vec<_>>>()?
			))
		};

	match (data_strings, data_types) {
		// only variants without fields
		(Some(data), None) => Ok(data),
		// only one variant with fields
		(None, Some(ParseData::Alternatives(mut alt))) if alt.len() == 1 => Ok(alt.remove(0)),
		// only variants with fields
		(None, Some(data)) => Ok(data),
		// variants with and without fields
		(Some(data), Some(ParseData::Alternatives(mut alt))) => {
			alt.push(data);
			Ok(ParseData::Alternatives(alt))
		},
		// no variants
		(None, None) => Err(syn::Error::new(
			inum.brace_token.span,
			"#[derive(OpenapiType)] does not support enums with no variants"
		)),
		// data_types always produces Alternatives
		_ => unreachable!()
	}
}

pub(super) fn parse_union(union: &DataUnion) -> syn::Result<ParseData> {
	Err(syn::Error::new(
		union.union_token.span(),
		"#[derive(OpenapiType)] cannot be used on unions"
	))
}

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
