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
			Self::Alternatives(alt) => gen_alt(alt),
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
					::openapi_type::private::add_dependencies(
						&mut dependencies,
						&mut field_schema.dependencies
					);

					// fields in OpenAPI are nullable by default
					match field_schema.nullable {
						true => field_schema.nullable = false,
						false => required.push(::std::string::String::from(FIELD_NAME))
					};

					match field_schema.name.as_ref() {
						// include the field schema as reference
						#option::Some(schema_name) => {
							let mut reference = ::std::string::String::from("#/components/schemas/");
							let ref_name = schema_name.replace(|c: ::core::primitive::char| !c.is_alphanumeric(), "_");
							reference.push_str(&ref_name);
							properties.insert(
								::std::string::String::from(FIELD_NAME),
								#openapi::ReferenceOr::Reference { reference }
							);
							dependencies.insert(
								ref_name,
								field_schema
							);
						},

						// inline the field schema
						#option::None => {
							let mut schema = field_schema.into_schema();
							schema.schema_data.description = FIELD_DOC.map(|desc| {
								::std::string::String::from(desc)
							});
							properties.insert(
								::std::string::String::from(FIELD_NAME),
								#openapi::ReferenceOr::Item(
									::std::boxed::Box::new(schema)
								)
							);
						}
					};
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

fn gen_alt(alt: &[ParseData]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	let schema = alt.iter().map(|data| data.gen_schema());
	quote! {
		{
			let mut alternatives = <::std::vec::Vec<
				#openapi::ReferenceOr<#openapi::Schema>
			>>::new();
			#(alternatives.push(#openapi::ReferenceOr::Item(#schema.into_schema()));)*

			::openapi_type::OpenapiSchema::new(
				#openapi::SchemaKind::OneOf {
					one_of: alternatives
				}
			)
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
