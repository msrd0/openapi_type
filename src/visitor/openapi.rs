use super::*;
use indexmap::{map::Entry, IndexMap};
use openapiv3::{ObjectType, ReferenceOr, Schema, SchemaData, SchemaKind, Type};

trait Boxed {
	type Boxed;

	fn boxed(self) -> Self::Boxed;
}

impl<T> Boxed for ReferenceOr<T> {
	type Boxed = ReferenceOr<Box<T>>;

	fn boxed(self) -> Self::Boxed {
		match self {
			ReferenceOr::Item(t) => ReferenceOr::Item(Box::new(t)),
			ReferenceOr::Reference { reference } => ReferenceOr::Reference { reference }
		}
	}
}

#[readonly::make]
#[derive(Debug)]
pub struct OpenapiSchema {
	pub schema: Schema,
	pub dependencies: IndexMap<String, OpenapiSchema>
}

#[derive(Debug)]
pub enum OpenapiVisitor {
	Empty,

	Object(Object)
}

impl OpenapiVisitor {
	pub const fn new() -> Self {
		Self::Empty
	}

	pub fn into_schema(self) -> Option<OpenapiSchema> {
		match self {
			Self::Empty => None,
			Self::Object(obj) => Some(obj.into_schema())
		}
	}

	#[track_caller]
	fn panic_non_empty(&self) -> ! {
		panic!("This visitor has been called before. You may only specify one type per visitor.");
	}
}

impl seal::Sealed for OpenapiVisitor {}

impl Visitor for OpenapiVisitor {
	type ObjectVisitor = Object;

	fn visit_object(&mut self) -> &mut Object {
		if !matches!(self, Self::Empty) {
			self.panic_non_empty();
		}
		*self = Self::Object(Object::default());
		match self {
			Self::Object(obj) => obj,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub struct Field {
	doc: Option<String>,
	visitor: OpenapiVisitor
}

#[derive(Debug, Default)]
pub struct Object {
	name: Option<String>,
	description: Option<String>,
	fields: IndexMap<String, Field>
}

fn add_dependencies(dependencies: &mut IndexMap<String, OpenapiSchema>, other: &mut IndexMap<String, OpenapiSchema>) {
	while let Some((name, schema)) = other.pop() {
		dependencies.entry(name).or_insert(schema);
	}
}

fn inline_if_unnamed(
	dependencies: &mut IndexMap<String, OpenapiSchema>,
	mut schema: OpenapiSchema,
	doc: Option<String>
) -> ReferenceOr<Schema> {
	add_dependencies(dependencies, &mut schema.dependencies);
	match schema.schema.schema_data.title.as_deref() {
		Some(schema_name) => {
			let ref_name = schema_name.replace(|c: char| !c.is_alphanumeric(), "_");
			let reference = format!("#/components/schemas/{ref_name}");
			dependencies.insert(ref_name, schema);
			ReferenceOr::Reference { reference }
		},
		None => {
			if let Some(doc) = doc {
				schema.schema.schema_data.description = Some(doc);
			}
			ReferenceOr::Item(schema.schema)
		}
	}
}

impl Object {
	fn into_schema(self) -> OpenapiSchema {
		let mut properties = IndexMap::new();
		let mut required = Vec::new();
		let mut dependencies = IndexMap::new();

		for (field_name, field) in self.fields {
			let mut schema = field
				.visitor
				.into_schema()
				.expect("Field {field_name:?} failed to call its type visitor");
			if !schema.schema.schema_data.nullable {
				required.push(field_name.clone());
			}
			schema.schema.schema_data.nullable = false;

			let field_schema = inline_if_unnamed(&mut dependencies, schema, field.doc);
			properties.insert(field_name, field_schema.boxed());
		}

		let schema = Schema {
			schema_data: SchemaData {
				title: self.name,
				description: self.description,
				..Default::default()
			},
			schema_kind: SchemaKind::Type(Type::Object(ObjectType {
				properties,
				required,
				..Default::default()
			}))
		};
		OpenapiSchema { schema, dependencies }
	}
}

impl seal::Sealed for Object {}

impl ObjectVisitor for Object {
	type FieldVisitor = OpenapiVisitor;

	fn visit_name(&mut self, name: String) {
		if self.name.is_some() {
			panic!("You must only set the name of this object once");
		}
		self.name = Some(name);
	}

	fn visit_description(&mut self, description: String) {
		if self.description.is_some() {
			panic!("You must only set the description of this object once");
		}
		self.description = Some(description);
	}

	fn visit_field(&mut self, name: String, doc: Option<String>) -> &mut Self::FieldVisitor {
		match self.fields.entry(name) {
			Entry::Occupied(entry) => panic!("You must only visit the field with name {:?} once", entry.key()),
			Entry::Vacant(entry) => {
				&mut entry
					.insert(Field {
						doc,
						visitor: OpenapiVisitor::new()
					})
					.visitor
			},
		}
	}
}
