use crate::{
	attrs::{parse_doc_attr, ContainerAttributes},
	serde_derive_internals::case::RenameRule,
	util::ToLitStr
};
use proc_macro2::Span;
use syn::{spanned::Spanned as _, DataEnum, DataStruct, DataUnion, Fields, FieldsNamed, LitStr, Type};

pub(super) enum TypeOrInline {
	Type(Type),
	Inline(ParseData)
}

pub(super) struct ParseDataField {
	pub(super) name: LitStr,
	pub(super) doc: Vec<String>,
	pub(super) ty: TypeOrInline
}

#[allow(dead_code)]
pub(super) enum ParseData {
	Struct(Vec<ParseDataField>),
	Enum(Vec<LitStr>),
	Alternatives(Vec<ParseData>),
	Unit
}

fn parse_named_fields(named_fields: &FieldsNamed, rename_all: Option<&LitStr>) -> syn::Result<ParseData> {
	let mut fields: Vec<ParseDataField> = Vec::new();
	for f in &named_fields.named {
		let mut doc = Vec::new();
		for attr in &f.attrs {
			if attr.path.is_ident("doc") {
				if let Some(lit) = parse_doc_attr(attr)? {
					doc.push(lit.value());
				}
			}
		}

		let ident = f
			.ident
			.as_ref()
			.ok_or_else(|| syn::Error::new(f.span(), "#[derive(OpenapiType)] does not support fields without an ident"))?;
		let mut name = ident.to_lit_str();
		if let Some(rename_all) = rename_all {
			let rule: RenameRule = rename_all
				.value()
				.parse()
				.map_err(|_| syn::Error::new(rename_all.span(), "Unknown rename_all rule"))?;
			let rename = rule.apply_to_field(&name.value());
			name = LitStr::new(&rename, name.span());
		}
		let ty = f.ty.to_owned();
		fields.push(ParseDataField {
			name,
			doc,
			ty: TypeOrInline::Type(ty)
		});
	}
	Ok(ParseData::Struct(fields))
}

pub(super) fn parse_struct(strukt: &DataStruct, attrs: &ContainerAttributes) -> syn::Result<ParseData> {
	match &strukt.fields {
		Fields::Named(named_fields) => parse_named_fields(named_fields, attrs.rename_all.as_ref()),
		Fields::Unnamed(unnamed_fields) => Err(syn::Error::new(
			unnamed_fields.span(),
			"#[derive(OpenapiType)] does not support tuple structs"
		)),
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
				types.push((name, parse_named_fields(named_fields, attrs.rename_all.as_ref())?));
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
			(Some(tag), _, false) => Some(ParseData::Struct(vec![ParseDataField {
				name: tag.clone(),
				doc: Vec::new(),
				ty: TypeOrInline::Inline(ParseData::Enum(strings))
			}])),
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
							(None, None, false) => ParseData::Struct(vec![ParseDataField {
								name,
								doc: Vec::new(),
								ty: TypeOrInline::Inline(data)
							}]),
							// internally tagged
							(Some(tag), None, false) => {
								match &mut data {
									ParseData::Struct(fields) => fields.push(ParseDataField {
										name: tag.clone(),
										doc: Vec::new(),
										ty: TypeOrInline::Inline(ParseData::Enum(vec![name]))
									}),
									_ => return Err(syn::Error::new(
										tag.span(),
										"#[derive(OpenapiType)] does not support tuple variants on internally tagged enums"
									))
								};
								data
							},
							// adjacently tagged
							(Some(tag), Some(content), false) => ParseData::Struct(vec![
								ParseDataField {
									name: tag.clone(),
									doc: Vec::new(),
									ty: TypeOrInline::Inline(ParseData::Enum(vec![name]))
								},
								ParseDataField {
									name: content.clone(),
									doc: Vec::new(),
									ty: TypeOrInline::Inline(data)
								},
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
