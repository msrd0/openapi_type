use crate::OpenapiSchema;
use indexmap::IndexMap;

pub type Dependencies = IndexMap<String, OpenapiSchema>;

pub fn add_dependencies(dependencies: &mut Dependencies, other: &mut Dependencies) {
	while let Some((dep_name, dep_schema)) = other.pop() {
		if !dependencies.contains_key(&dep_name) {
			dependencies.insert(dep_name, dep_schema);
		}
	}
}
