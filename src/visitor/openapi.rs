use super::*;
use indexmap::{map::Entry, IndexMap};
use openapiv3::{
	AdditionalProperties, IntegerFormat, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr, Schema, SchemaData,
	SchemaKind, StringFormat, StringType, Type, VariantOrUnknownOrEmpty
};

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

impl OpenapiSchema {
	fn new(schema: Schema) -> Self {
		Self {
			schema,
			dependencies: IndexMap::new()
		}
	}
}

#[derive(Debug)]
pub enum OpenapiVisitor {
	Empty,

	Unit,
	Any,
	Bool,

	Int { byte: Option<u32>, minimum: Option<i64> },
	Number { byte: Option<u32> },
	Char,

	String,
	Uuid,
	Date,
	DateTime,

	Option(Box<OpenapiVisitor>),

	Object(Object)
}

impl OpenapiVisitor {
	pub const fn new() -> Self {
		Self::Empty
	}

	pub fn into_schema(self) -> Option<OpenapiSchema> {
		match self {
			Self::Empty => None,

			Self::Unit => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::Object(ObjectType {
					additional_properties: Some(AdditionalProperties::Any(false)),
					..Default::default()
				}))
			})),

			Self::Any => Some(OpenapiSchema::new(Schema {
				schema_data: SchemaData {
					nullable: true,
					..Default::default()
				},
				schema_kind: SchemaKind::Any(Default::default())
			})),

			Self::Bool => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::Boolean {})
			})),

			Self::Int { byte, minimum } => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::Integer(IntegerType {
					format: match byte {
						None => VariantOrUnknownOrEmpty::Empty,
						Some(4) => VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
						Some(8) => VariantOrUnknownOrEmpty::Item(IntegerFormat::Int64),
						Some(byte) => VariantOrUnknownOrEmpty::Unknown(format!("int{}", byte * 8))
					},
					minimum,
					..Default::default()
				}))
			})),

			Self::Number { byte } => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::Number(NumberType {
					format: match byte {
						None => VariantOrUnknownOrEmpty::Empty,
						Some(4) => VariantOrUnknownOrEmpty::Item(NumberFormat::Float),
						Some(8) => VariantOrUnknownOrEmpty::Item(NumberFormat::Double),
						Some(byte) => VariantOrUnknownOrEmpty::Unknown(format!("f{}", byte * 8))
					},
					..Default::default()
				}))
			})),

			Self::Char => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::String(StringType {
					min_length: Some(1),
					max_length: Some(1),
					..Default::default()
				}))
			})),

			Self::String => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::String(Default::default()))
			})),

			Self::Uuid => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::String(StringType {
					format: VariantOrUnknownOrEmpty::Unknown("uuid".into()),
					..Default::default()
				}))
			})),

			Self::Date => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::String(StringType {
					format: VariantOrUnknownOrEmpty::Item(StringFormat::Date),
					..Default::default()
				}))
			})),

			Self::DateTime => Some(OpenapiSchema::new(Schema {
				schema_data: Default::default(),
				schema_kind: SchemaKind::Type(Type::String(StringType {
					format: VariantOrUnknownOrEmpty::Item(StringFormat::DateTime),
					..Default::default()
				}))
			})),

			Self::Option(opt) => opt.into_schema().map(|mut schema| match schema.schema.schema_data.title {
				Some(title) => {
					schema.dependencies.insert(title.clone(), schema);
					OpenapiSchema {
						schema: Schema {
							schema_data: SchemaData {
								nullable: true,
								..Default::default()
							},
							schema_kind: SchemaKind::AllOf {
								all_of: vec![ReferenceOr::Reference {
									reference: format!("#/components/schemas/{title}")
								}]
							}
						},
						dependencies: schema.dependencies
					}
				},
				None => {
					schema.schema.schema_data.nullable = true;
					schema
				}
			}),

			Self::Object(obj) => Some(obj.into_schema())
		}
	}

	#[track_caller]
	fn panic_if_non_empty(&self) {
		if !matches!(self, Self::Empty) {
			panic!("This visitor has been called before. You may only specify one type per visitor.");
		}
	}
}

impl seal::Sealed for OpenapiVisitor {}

impl Visitor for OpenapiVisitor {
	type OptionVisitor = Self;
	type ObjectVisitor = Object;

	fn visit_unit(&mut self) {
		self.panic_if_non_empty();
		*self = Self::Unit;
	}

	fn visit_any(&mut self) {
		self.panic_if_non_empty();
		*self = Self::Any;
	}

	fn visit_bool(&mut self) {
		self.panic_if_non_empty();
		*self = Self::Bool;
	}

	fn visit_int(&mut self, byte: Option<u32>, minimum: Option<i64>) {
		self.panic_if_non_empty();
		*self = Self::Int { byte, minimum };
	}

	fn visit_number(&mut self, byte: Option<u32>) {
		self.panic_if_non_empty();
		*self = Self::Number { byte }
	}

	fn visit_char(&mut self) {
		self.panic_if_non_empty();
		*self = Self::Char;
	}

	fn visit_string(&mut self) {
		self.panic_if_non_empty();
		*self = Self::String;
	}

	fn visit_uuid(&mut self) {
		self.panic_if_non_empty();
		*self = Self::Uuid;
	}

	fn visit_date(&mut self) {
		self.panic_if_non_empty();
		*self = Self::Date;
	}

	fn visit_datetime(&mut self) {
		self.panic_if_non_empty();
		*self = Self::DateTime;
	}

	fn visit_option(&mut self) -> &mut Self {
		self.panic_if_non_empty();
		*self = Self::Option(Box::new(Self::new()));
		match self {
			Self::Option(opt) => opt,
			_ => unreachable!()
		}
	}

	fn visit_object(&mut self) -> &mut Object {
		self.panic_if_non_empty();
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
