use crate::OpenapiSchema;
use indexmap::IndexMap;
use openapiv3::{ReferenceOr, Schema};

pub type Dependencies = IndexMap<String, OpenapiSchema>;

fn add_dependencies(dependencies: &mut Dependencies, other: &mut Dependencies) {
	while let Some((dep_name, dep_schema)) = other.pop() {
		if !dependencies.contains_key(&dep_name) {
			dependencies.insert(dep_name, dep_schema);
		}
	}
}

/// Add the schema to the dependencies and always return a reference. The schema's dependencies
/// are always added to the dependencies.
pub fn reference<T, F>(dependencies: &mut Dependencies, mut schema: OpenapiSchema, fallback_ref_name: F) -> ReferenceOr<T>
where
	F: FnOnce() -> String
{
	add_dependencies(dependencies, &mut schema.dependencies);

	let ref_name = schema
		.name
		.as_deref()
		.map(|name| name.replace(|c: char| !c.is_alphanumeric(), "_"))
		.unwrap_or_else(fallback_ref_name);
	let reference = format!("#/components/schemas/{}", ref_name);

	dependencies.insert(ref_name, schema);
	ReferenceOr::Reference { reference }
}

/// Inline the schema if it is unnamed, otherwise return a reference to it and add it to the
/// dependencies. The schema's dependencies are always added to the dependencies.
pub fn inline_if_unnamed(
	dependencies: &mut Dependencies,
	mut schema: OpenapiSchema,
	doc: Option<&'static str>
) -> ReferenceOr<Schema> {
	match &schema.name {
		Some(_) => reference(dependencies, schema, || unreachable!()),

		None => {
			add_dependencies(dependencies, &mut schema.dependencies);
			let mut schema = schema.into_schema();
			if let Some(doc) = doc {
				schema.schema_data.description = Some(doc.to_string());
			}
			ReferenceOr::Item(schema)
		}
	}
}
