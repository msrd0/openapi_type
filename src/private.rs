use crate::OpenapiSchema;
use indexmap::IndexMap;
use openapiv3::{ReferenceOr, Schema, SchemaKind, Type};
use std::borrow::Cow;

pub type Dependencies = IndexMap<String, OpenapiSchema>;
pub type Properties = IndexMap<String, ReferenceOr<Box<Schema>>>;
pub type Required = Vec<String>;

fn add_dependencies(dependencies: &mut Dependencies, other: &mut Dependencies) {
	while let Some((dep_name, dep_schema)) = other.pop() {
		if !dependencies.contains_key(&dep_name) {
			dependencies.insert(dep_name, dep_schema);
		}
	}
}

pub fn inline_if_unnamed(
	dependencies: &mut Dependencies,
	mut schema: OpenapiSchema,
	doc: Option<&'static str>
) -> ReferenceOr<Schema> {
	add_dependencies(dependencies, &mut schema.dependencies);
	match schema.name.as_ref() {
		Some(schema_name) => {
			let ref_name = schema_name.replace(|c: char| !c.is_alphanumeric(), "_");
			let mut reference = "#/components/schemas/".to_string();
			reference.push_str(&ref_name);
			dependencies.insert(ref_name, schema);
			ReferenceOr::Reference { reference }
		},

		None => {
			let mut schema = schema.into_schema();
			if let Some(doc) = doc {
				schema.schema_data.description = Some(doc.to_string());
			}
			ReferenceOr::Item(schema)
		}
	}
}

struct FlattenError(Cow<'static, str>);

fn flatten_impl(properties: &mut Properties, required: &mut Required, schema: SchemaKind) -> Result<(), FlattenError> {
	let mut obj = match schema {
		SchemaKind::Type(Type::Object(obj)) => obj,
		SchemaKind::OneOf { .. } => {
			return Err(FlattenError(
				"#[serde(flatten)] is currently not supported for enums with non-unit variants".into()
			))
		},
		_ => return Err(FlattenError("Expected object".into()))
	};

	while let Some((prop_name, prop_schema)) = obj.properties.pop() {
		if properties.contains_key(&prop_name) {
			return Err(FlattenError(format!("Duplicate property name {}", prop_name).into()));
		}
		properties.insert(prop_name, prop_schema);
	}
	required.extend(obj.required.into_iter());

	Ok(())
}

pub fn flatten(
	dependencies: &mut Dependencies,
	properties: &mut Properties,
	required: &mut Required,
	mut schema: OpenapiSchema
) {
	add_dependencies(dependencies, &mut schema.dependencies);
	match flatten_impl(properties, required, schema.schema) {
		Ok(_) => {},
		Err(e) => panic!("Flattening produced an error: {}", e.0)
	};
}
