use crate::parser::{ParseData, ParseDataType};
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

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

fn gen_struct(fields: &[(LitStr, ParseDataType)]) -> TokenStream {
	let field_name = fields.iter().map(|(name, _)| name);
	let field_schema = fields.iter().map(|(_, ty)| match ty {
		ParseDataType::Type(ty) => {
			quote!(<#ty as ::openapi_type::OpenapiType>::schema())
		},
		ParseDataType::Inline(data) => {
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
							properties.insert(
								::std::string::String::from(FIELD_NAME),
								#openapi::ReferenceOr::Item(
									::std::boxed::Box::new(
										field_schema.into_schema()
									)
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

fn gen_alt(alt: &[ParseData]) -> TokenStream {
	let openapi = path!(::openapi_type::openapi);
	let schema = alt.iter().map(|data| data.gen_schema());
	quote! {
		{
			let mut alternatives = <::std::vec::Vec<
				#openapi::ReferenceOr<#openapi::Schema>
			>>::new();
			#(alternatives.push(#openapi::ReferenceOr::Item(
				::openapi_type::OpenapiSchema::new(#schema).into_schema()
			));)*
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
