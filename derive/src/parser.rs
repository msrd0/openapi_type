use crate::{
	attrs::{parse_doc_attr, ContainerAttributes, FieldAttributes},
	util::ToLitStr,
};
use proc_macro2::{Ident, Span};
use serde_derive_internals::attr::RenameRule;
use syn::{
	punctuated::Punctuated, spanned::Spanned as _, AngleBracketedGenericArguments, DataEnum, DataStruct, DataUnion, Fields,
	FieldsNamed, GenericArgument, LitStr, PathArguments, Type, TypePath,
};

pub(super) enum TypeOrInline {
	Type(Type),
	Inline(ParseData),
}

pub(super) struct ParseDataField {
	pub(super) name: LitStr,
	pub(super) doc: Vec<String>,
	pub(super) ty: TypeOrInline,
}

#[allow(dead_code)]
pub(super) enum ParseData {
	Struct {
		name: Option<LitStr>,
		fields: Vec<ParseDataField>,
	},
	Enum(Vec<LitStr>),
	Alternatives(Vec<ParseData>),
	Unit,
}

fn parse_named_fields(named_fields: &FieldsNamed, rename_all: Option<&LitStr>) -> syn::Result<Vec<ParseDataField>> {
	let mut fields: Vec<ParseDataField> = Vec::new();
	for f in &named_fields.named {
		// parse #[serde] and #[openapi] attributes
		let mut attrs = FieldAttributes::default();
		for attr in &f.attrs {
			if attr.path.is_ident("serde") {
				attrs.parse_from(attr, false)?;
			}
		}
		for attr in &f.attrs {
			if attr.path.is_ident("openapi") {
				attrs.parse_from(attr, true)?;
			}
		}

		// skip this field if desired
		if attrs.skip_serializing && attrs.skip_deserializing {
			continue;
		}

		// parse #[doc] attributes
		let mut doc = Vec::new();
		for attr in &f.attrs {
			if attr.path.is_ident("doc") {
				if let Some(lit) = parse_doc_attr(attr)? {
					doc.push(lit.value());
				}
			}
		}

		// get the name of the field
		let ident = f
			.ident
			.as_ref()
			.ok_or_else(|| syn::Error::new(f.span(), "#[derive(OpenapiType)] does not support fields without an ident"))?;
		let mut name = ident.to_lit_str();
		if let Some(rename) = attrs.rename {
			name = rename;
		} else if let Some(rename_all) = rename_all {
			let rule = RenameRule::from_str(&rename_all.value())
				.map_err(|_| syn::Error::new(rename_all.span(), "Unknown rename_all rule"))?;
			let rename = rule.apply_to_field(&name.value());
			name = LitStr::new(&rename, name.span());
		}

		// get the type of the field
		let mut ty = f.ty.to_owned();
		if attrs.nullable {
			let mut args = Punctuated::new();
			args.push(GenericArgument::Type(ty));
			let mut path = path!(::core::option::Option);
			let last = path.segments.last_mut().unwrap();
			last.arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
				colon2_token: None,
				lt_token: Default::default(),
				args,
				gt_token: Default::default(),
			});
			ty = Type::Path(TypePath { qself: None, path })
		}

		fields.push(ParseDataField {
			name,
			doc,
			ty: TypeOrInline::Type(ty),
		});
	}
	Ok(fields)
}

pub(super) fn parse_struct(ident: &Ident, strukt: &DataStruct, attrs: &ContainerAttributes) -> syn::Result<ParseData> {
	match &strukt.fields {
		Fields::Named(named_fields) => {
			let fields = parse_named_fields(named_fields, attrs.rename_all.as_ref())?;
			Ok(ParseData::Struct {
				name: Some(ident.to_lit_str()),
				fields,
			})
		}
		Fields::Unnamed(unnamed_fields) => Err(syn::Error::new(
			unnamed_fields.span(),
			"#[derive(OpenapiType)] does not support tuple structs",
		)),
		Fields::Unit => Ok(ParseData::Unit),
	}
}

pub(super) fn parse_enum(ident: &Ident, inum: &DataEnum, attrs: &ContainerAttributes) -> syn::Result<ParseData> {
	let mut strings: Vec<LitStr> = Vec::new();
	let mut types: Vec<(LitStr, ParseData)> = Vec::new();

	for v in &inum.variants {
		let name = v.ident.to_lit_str();
		match &v.fields {
			Fields::Named(named_fields) => {
				let fields = parse_named_fields(named_fields, attrs.rename_all.as_ref())?;
				let struct_name = format!("{}::{}", ident, name.value());
				types.push((
					name,
					ParseData::Struct {
						name: Some(struct_name.to_lit_str()),
						fields,
					},
				));
			}
			Fields::Unnamed(unnamed_fields) => {
				return Err(syn::Error::new(
					unnamed_fields.span(),
					"#[derive(OpenapiType)] does not support tuple variants",
				))
			}
			Fields::Unit => strings.push(name),
		}
	}

	let data_strings = if strings.is_empty() {
		None
	} else {
		match (&attrs.tag, &attrs.content, attrs.untagged) {
			// externally tagged (default)
			(None, None, false) => Some(ParseData::Enum(strings)),
			// internally tagged or adjacently tagged
			(Some(tag), _, false) => Some(ParseData::Struct {
				name: None,
				fields: vec![ParseDataField {
					name: tag.clone(),
					doc: Vec::new(),
					ty: TypeOrInline::Inline(ParseData::Enum(strings)),
				}],
			}),
			// untagged
			(None, None, true) => Some(ParseData::Unit),
			// unknown
			_ => return Err(syn::Error::new(Span::call_site(), "Unknown enum representation")),
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
							(None, None, false) => {
								let struct_name = format!("{}::{}::ExtTagWrapper", ident, name.value());
								ParseData::Struct {
									name: Some(struct_name.to_lit_str()),
									fields: vec![ParseDataField {
										name,
										doc: Vec::new(),
										ty: TypeOrInline::Inline(data),
									}],
								}
							}
							// internally tagged
							(Some(tag), None, false) => {
								match &mut data {
									ParseData::Struct { fields, .. } => fields.push(ParseDataField {
										name: tag.clone(),
										doc: Vec::new(),
										ty: TypeOrInline::Inline(ParseData::Enum(vec![name])),
									}),
									_ => return Err(syn::Error::new(
										tag.span(),
										"#[derive(OpenapiType)] does not support tuple variants on internally tagged enums",
									)),
								};
								data
							}
							// adjacently tagged
							(Some(tag), Some(content), false) => {
								let struct_name = format!("{}::{}::AdjTagWrapper", ident, name.value());
								ParseData::Struct {
									name: Some(struct_name.to_lit_str()),
									fields: vec![
										ParseDataField {
											name: tag.clone(),
											doc: Vec::new(),
											ty: TypeOrInline::Inline(ParseData::Enum(vec![name])),
										},
										ParseDataField {
											name: content.clone(),
											doc: Vec::new(),
											ty: TypeOrInline::Inline(data),
										},
									],
								}
							}
							// untagged
							(None, None, true) => data,
							// unknown
							_ => return Err(syn::Error::new(Span::call_site(), "Unknown enum representation")),
						})
					})
					.collect::<syn::Result<Vec<_>>>()?,
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
		}
		// no variants
		(None, None) => Err(syn::Error::new(
			inum.brace_token.span,
			"#[derive(OpenapiType)] does not support enums with no variants",
		)),
		// data_types always produces Alternatives
		_ => unreachable!(),
	}
}

pub(super) fn parse_union(union: &DataUnion) -> syn::Result<ParseData> {
	Err(syn::Error::new(
		union.union_token.span(),
		"#[derive(OpenapiType)] cannot be used on unions",
	))
}
