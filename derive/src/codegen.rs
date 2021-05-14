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
			Self::Struct(fields) => gen_struct(fields),
			Self::Enum(variants) => gen_enum(variants),
			Self::Alternatives(alt) => gen_alt(alt),
			Self::Unit => gen_unit()
		}
	}
}

fn gen_struct(fields: &[ParseDataField]) -> TokenStream {
	let field_name = fields.iter().map(|f| &f.name);
	let field_doc = fields.iter().map(|f| gen_doc_option(&f.doc));
	let field_schema = fields.iter().map(|f| match &f.ty {
		TypeOrInline::Type(ty) => {
			quote!(<#ty as ::openapi_type::OpenapiType>::schema())
		},
		TypeOrInline::Inline(data) => {
			let code = data.gen_schema();
			quote!(::openapi_type::OpenapiSchema::new(#code))
		}
	});

	let openapi = path!(::openapi_type::openapi);
	quote! {
		{
			let mut properties = <::openapi_type::indexmap::IndexMap<
				::std::string::String,
				#openapi::ReferenceOr<::std::boxed::Box<#openapi::Schema>>
			>>::new();
			let mut required = <::std::vec::Vec<::std::string::String>>::new();

			#({
					const FIELD_NAME: &::core::primitive::str = #field_name;
					const FIELD_DOC: ::core::option::Option<&'static ::core::primitive::str> = #field_doc;

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
						::std::option::Option::Some(schema_name) => {
							let mut reference = ::std::string::String::from("#/components/schemas/");
							reference.push_str(schema_name);
							properties.insert(
								::std::string::String::from(FIELD_NAME),
								#openapi::ReferenceOr::Reference { reference }
							);
							dependencies.insert(
								::std::string::String::from(schema_name),
								field_schema
							);
						},

						// inline the field schema
						::std::option::Option::None => {
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
					}
			})*

			#openapi::SchemaKind::Type(
				#openapi::Type::Object(
					#openapi::ObjectType {
						properties,
						required,
						.. ::std::default::Default::default()
					}
				)
			)
		}
	}
}

fn gen_enum(variants: &[LitStr]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	quote! {
		{
			let mut enumeration = <::std::vec::Vec<::std::string::String>>::new();
			#(enumeration.push(::std::string::String::from(#variants));)*
			#openapi::SchemaKind::Type(
				#openapi::Type::String(
					#openapi::StringType {
						enumeration,
						.. ::std::default::Default::default()
					}
				)
			)
		}
	}
}

fn gen_alt(alt: &[(Option<LitStr>, ParseData)]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	let option = path!(::core::option::Option);
	let variant_name = alt.iter().map(|data| {
		data.0
			.as_ref()
			.map(|name| quote!(#option::Some(#name)))
			.unwrap_or_else(|| quote!(#option::None))
	});
	let schema = alt.iter().map(|data| data.1.gen_schema());
	quote! {
		{
			let mut alternatives = <::std::vec::Vec<
				#openapi::ReferenceOr<#openapi::Schema>
			>>::new();
			#(alternatives.push(#openapi::ReferenceOr::Item({
				let mut variant_schema = ::openapi_type::OpenapiSchema::new(#schema).into_schema();
				const VARIANT_NAME: #option<&'static ::core::primitive::str> = #variant_name;
				if let #option::Some(variant_name) = VARIANT_NAME {
					let variant_title = #option::Some(format!("{}::{}", NAME, variant_name));
					variant_schema.schema_data.title = variant_title;
				}
				variant_schema
			}));)*
			#openapi::SchemaKind::OneOf {
				one_of: alternatives
			}
		}
	}
}

fn gen_unit() -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	quote! {
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
	}
}
