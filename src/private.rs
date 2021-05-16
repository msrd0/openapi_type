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
