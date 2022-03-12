use crate::parser::{ParseData, ParseDataField, ParseDataType, TypeOrInline};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, LitStr};

fn gen_doc_option(doc: &[String]) -> TokenStream {
	let doc = doc.join("\n");
	let doc = doc.trim();
	if doc.is_empty() {
		quote!(::core::option::Option::None)
	} else {
		quote!(::core::option::Option::Some(#doc))
	}
}

impl ParseData {
	pub(super) fn gen_visit_impl(&self) -> TokenStream {
		let name = self.name.as_ref();
		let doc = &self.doc;
		match &self.ty {
			ParseDataType::Struct {
				fields,
				deny_unknown_fields
			} => gen_struct(name, doc, fields, *deny_unknown_fields),
			ParseDataType::Enum { variants } => gen_enum(name, doc, variants),
			ParseDataType::Alternatives { alts } => gen_alt(name, doc, alts),
			ParseDataType::Unit => gen_unit(name, doc)
		}
	}
}

impl TypeOrInline {
	fn visit_type_fn(&self) -> TokenStream {
		match self {
			Self::Type(ty) => {
				quote_spanned!(ty.span() => <#ty as ::openapi_type::OpenapiType>::visit_type)
			},
			Self::Inline(data) => {
				let visit_impl = data.gen_visit_impl();
				quote! {
					{
						fn visit_type<__openapi_type_V>(visitor: &mut __openapi_type_V)
						where
							__openapi_type_V: ::openapi_type::Visitor
						{
							#visit_impl
						}
						visit_type
					}
				}
			}
		}
	}
}

fn gen_struct(name: Option<&LitStr>, doc: &[String], fields: &[ParseDataField], deny_unknown_fields: bool) -> TokenStream {
	let str = path!(::core::primitive::str);
	let string = path!(::std::string::String);
	let option = path!(::core::option::Option);

	let name = match name {
		Some(name) => quote!(#option::Some(#name)),
		None => quote!(#option::None)
	};
	let doc = gen_doc_option(doc);

	let fields = fields.iter().map(|f| {
		let name = &f.name;
		let doc = gen_doc_option(&f.doc);
		let visit = f.ty.visit_type_fn();

		if f.flatten {
			quote!({
				let visitor = ::openapi_type::ObjectVisitor::visit_flatten_field(object_visitor);
				let field_visit_type = #visit;
				field_visit_type(visitor)
			})
		} else {
			quote!({
				const FIELD_NAME: &#str = #name;
				const FIELD_DOC: #option<&'static #str> = #doc;

				let field_visit_type = #visit;
				let field_visitor = ::openapi_type::ObjectVisitor::visit_field(
					object_visitor,
					#string::from(FIELD_NAME),
					FIELD_DOC.map(#string::from)
				);
				field_visit_type(field_visitor);
			})
		}
	});

	quote! {
		const OBJECT_NAME: #option<&'static #str> = #name;
		const OBJECT_DOC: #option<&'static #str> = #doc;

		let object_visitor = ::openapi_type::Visitor::visit_object(visitor);

		if let #option::Some(object_name) = OBJECT_NAME {
			::openapi_type::ObjectVisitor::visit_name(
				object_visitor,
				#string::from(object_name)
			);
		}
		if let #option::Some(object_doc) = OBJECT_DOC {
			::openapi_type::ObjectVisitor::visit_description(
				object_visitor,
				#string::from(object_doc)
			);
		}
		if #deny_unknown_fields {
			::openapi_type::ObjectVisitor::visit_deny_additional(object_visitor);
		}

		#(#fields)*
	}
}

fn gen_enum(name: Option<&LitStr>, doc: &[String], variants: &[LitStr]) -> TokenStream {
	let str = path!(::core::primitive::str);
	let string = path!(::std::string::String);
	let option = path!(::core::option::Option);

	let name = match name {
		Some(name) => quote!(#option::Some(#name)),
		None => quote!(#option::None)
	};
	let doc = gen_doc_option(doc);

	quote! {
		const ENUM_NAME: #option<&'static #str> = #name;
		const ENUM_DOC: #option<&'static #str> = #doc;

		::openapi_type::Visitor::visit_enum(
			visitor,
			ENUM_NAME.map(#string::from),
			ENUM_DOC.map(#string::from),
			[#(#string::from(#variants)),*]
		);
	}
}

fn gen_alt(name: Option<&LitStr>, doc: &[String], alt: &[TypeOrInline]) -> TokenStream {
	let str = path!(::core::primitive::str);
	let string = path!(::std::string::String);
	let option = path!(::core::option::Option);

	let name = match name {
		Some(name) => quote!(#option::Some(#name)),
		None => quote!(#option::None)
	};
	let doc = gen_doc_option(doc);

	let fns = alt.into_iter().map(|alt| alt.visit_type_fn());
	quote! {
		const OBJECT_NAME: #option<&'static #str> = #name;
		const OBJECT_DOC: #option<&'static #str> = #doc;

		let alt_visitor = ::openapi_type::Visitor::visit_alternatives(visitor);

		if let #option::Some(object_name) = OBJECT_NAME {
			::openapi_type::AlternativesVisitor::visit_name(
				alt_visitor,
				#string::from(object_name)
			);
		}
		if let #option::Some(object_doc) = OBJECT_DOC {
			::openapi_type::AlternativesVisitor::visit_description(
				alt_visitor,
				#string::from(object_doc)
			);
		}

		#({
			let visitor = ::openapi_type::AlternativesVisitor::visit_alternative(alt_visitor);
			let visit_type_fn = #fns;
			visit_type_fn(visitor);
		})*
	}
}

fn gen_unit(name: Option<&LitStr>, doc: &[String]) -> TokenStream {
	let str = path!(::core::primitive::str);
	let string = path!(::std::string::String);
	let option = path!(::core::option::Option);

	let name = match name {
		Some(name) => quote!(#option::Some(#name)),
		None => quote!(#option::None)
	};
	let doc = gen_doc_option(doc);

	quote! {
		const OBJECT_NAME: #option<&'static #str> = #name;
		const OBJECT_DOC: #option<&'static #str> = #doc;

		let option_visitor = ::openapi_type::Visitor::visit_unit_struct(
			visitor,
			OBJECT_NAME.map(#string::from),
			OBJECT_DOC.map(#string::from)
		);
	}
}
