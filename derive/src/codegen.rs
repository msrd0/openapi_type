use crate::parser::{ParseData, ParseDataField, TypeOrInline};
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

pub(super) fn gen_doc_option(doc: &[String]) -> TokenStream {
	let doc = doc.join("\n");
	let doc = doc.trim();
	if doc.is_empty() {
		quote!(::core::option::Option::None)
	} else {
		quote!(::core::option::Option::Some(#doc))
	}
}

impl ParseData {
	pub(super) fn gen_schema(&self) -> TokenStream {
		match self {
			Self::Struct { name, fields } => gen_struct(name.as_ref(), fields),
			Self::Enum(variants) => gen_enum(variants),
			Self::Alternatives {
				alternatives,
				discriminator,
				mapping
			} => gen_alt(alternatives, discriminator.as_ref(), mapping),
			Self::Unit => gen_unit()
		}
	}
}

fn gen_struct(name: Option<&LitStr>, fields: &[ParseDataField]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	let option = path!(::core::option::Option);

	let name = match name {
		Some(name) => quote!(#option::Some(::std::string::String::from(#name))),
		None => quote!(#option::None)
	};

	let field_name = fields.iter().map(|f| &f.name);
	let field_doc = fields.iter().map(|f| gen_doc_option(&f.doc));
	let field_schema = fields.iter().map(|f| match &f.ty {
		TypeOrInline::Type(ty) => {
			quote!(<#ty as ::openapi_type::OpenapiType>::schema())
		},
		TypeOrInline::Inline(data) => data.gen_schema()
	});

	quote! {
		{
			let mut properties = <::openapi_type::indexmap::IndexMap<
				::std::string::String,
				#openapi::ReferenceOr<::std::boxed::Box<#openapi::Schema>>
			>>::new();
			let mut required = <::std::vec::Vec<::std::string::String>>::new();

			#({
					const FIELD_NAME: &::core::primitive::str = #field_name;
					const FIELD_DOC: #option<&'static ::core::primitive::str> = #field_doc;

					let mut field_schema = #field_schema;

					// fields in OpenAPI are nullable by default
					match field_schema.nullable {
						true => field_schema.nullable = false,
						false => required.push(::std::string::String::from(FIELD_NAME))
					};

					let field_schema = match ::openapi_type::private::inline_if_unnamed(
						&mut dependencies, field_schema, FIELD_DOC
					) {
						#openapi::ReferenceOr::Item(schema) =>
							#openapi::ReferenceOr::Item(::std::boxed::Box::new(schema)),
						#openapi::ReferenceOr::Reference { reference } =>
							#openapi::ReferenceOr::Reference { reference }
					};

					properties.insert(
						::std::string::String::from(FIELD_NAME),
						field_schema
					);
			})*

			let mut schema = ::openapi_type::OpenapiSchema::new(
				#openapi::SchemaKind::Type(
					#openapi::Type::Object(
						#openapi::ObjectType {
							properties,
							required,
							.. ::std::default::Default::default()
						}
					)
				)
			);
			schema.name = #name;
			schema
		}
	}
}

fn gen_enum(variants: &[LitStr]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	quote! {
		{
			let mut enumeration = <::std::vec::Vec<::std::string::String>>::new();
			#(enumeration.push(::std::string::String::from(#variants));)*
			::openapi_type::OpenapiSchema::new(
				#openapi::SchemaKind::Type(
					#openapi::Type::String(
						#openapi::StringType {
							enumeration,
							.. ::std::default::Default::default()
						}
					)
				)
			)
		}
	}
}

fn gen_alt(alt: &[ParseData], discriminator: Option<&LitStr>, mapping: &[(LitStr, LitStr)]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);

	let discriminator = match discriminator {
		Some(prop) => quote!(::core::option::Option::Some(#prop)),
		None => quote!(::core::option::Option::None)
	};
	let mapping_key = mapping.iter().map(|(key, _)| key);
	let mapping_val = mapping.iter().map(|(_, val)| val);

	let schema = alt.iter().map(|data| data.gen_schema());
	let index = alt.iter().enumerate().map(|(i, _)| i);
	quote! {
		{
			let mut alternatives = <::std::vec::Vec<
				#openapi::ReferenceOr<#openapi::Schema>
			>>::new();
			#(alternatives.push({
				let alt_schema = #schema;
				let alt_prop_len = match &alt_schema.schema {
					#openapi::SchemaKind::Type(#openapi::Type::Object(obj)) => obj.properties.len(),
					_ => 0
				};
				// $ref-erence all non-trivial objects
				if alt_prop_len > 0 {
					::openapi_type::private::reference(
						&mut dependencies,
						alt_schema,
						|| ::std::format!("{}_oneOf_{}", NAME, #index)
					)
				} else {
					::openapi_type::private::inline_if_unnamed(
						&mut dependencies,
						alt_schema,
						None
					)
				}
			});)*

			let mut schema = ::openapi_type::OpenapiSchema::new(
				#openapi::SchemaKind::OneOf {
					one_of: alternatives
				}
			);
			schema.discriminator = #discriminator.map(|prop_name: &::core::primitive::str| {
				let mut mapping = <::openapi_type::indexmap::IndexMap<
					::std::string::String, ::std::string::String
				>>::new();
				#(mapping.insert(
					::std::string::String::from(#mapping_key),
					::std::string::String::from(#mapping_val)
				);)*
				#openapi::Discriminator {
					property_name: ::std::string::String::from(prop_name),
					mapping,
					extensions: ::std::default::Default::default()
				}
			});
			schema
		}
	}
}

fn gen_unit() -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	quote! {
		::openapi_type::OpenapiSchema::new(
			#openapi::SchemaKind::Type(
				#openapi::Type::Object(
					#openapi::ObjectType {
						additional_properties: ::std::option::Option::Some(
							#openapi::AdditionalProperties::Any(false)
						),
						.. ::std::default::Default::default()
					}
				)
			)
		)
	}
}
