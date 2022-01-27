use super::*;
use openapiv3::{Schema, SchemaData, Type, ObjectType, SchemaKind};
use indexmap::{IndexMap, map::Entry};
use std::collections::HashMap;

pub struct OpenapiSchema {
	pub schema: Schema,
	pub dependencies: HashMap<String, Schema>
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

#[derive(Debug, Default)]
pub struct Object {
	name: Option<String>,
	description: Option<String>,
	fields: IndexMap<String, OpenapiVisitor>
}

impl Object {
	fn into_schema(self) -> OpenapiSchema {
		let obj = ObjectType {
			
		};
		let dependencies = HashMap::new();
		let schema = Schema {
			schema_data: SchemaData {
				title: self.name,
				description: self.description,
				..Default::default()
			},
			schema_kind: SchemaKind::Type(Type::Object(obj))
		};
		OpenapiSchema {
			schema, dependencies
		}
	}
}

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
	
	fn visit_field(&mut self, name: String) -> &mut Self::FieldVisitor {
		match self.fields.entry(name) {
			Entry::Occupied(entry) => panic!("You must only visit the field with name {:?} once", entry.key()),
			Entry::Vacant(entry) => entry.insert(OpenapiVisitor::new())
		}
	}
}
