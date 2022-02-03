use super::{never::Never, seal, AlternativesVisitor, ObjectVisitor, Visitor};
use indexmap::{map::Entry, IndexMap};
use openapiv3::{
	AdditionalProperties, ArrayType, IntegerFormat, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr, Schema,
	SchemaData, SchemaKind, StringFormat, StringType, Type, VariantOrUnknownOrEmpty
};
use std::fmt::Display;

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

#[derive(Debug)]
#[non_exhaustive]
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

	Unit {
		name: Option<String>,
		description: Option<String>
	},
	Any,
	Bool,

	Int {
		byte: Option<u32>,
		minimum: Option<i64>
	},
	Number {
		byte: Option<u32>
	},
	Char,

	String,
	Uuid,
	Date,
	DateTime,

	Option(Box<OpenapiVisitor>),
	Enum {
		name: Option<String>,
		description: Option<String>,
		variants: Vec<Option<String>>
	},
	Array {
		items: Box<OpenapiVisitor>,
		len: Option<usize>,
		unique_items: bool
	},
	Object(Object),
	Alternatives(Alternatives)
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

impl OpenapiVisitor {
	pub const fn new() -> Self {
		Self::Empty
	}

	pub fn into_schema(self) -> Option<OpenapiSchema> {
		match self {
			Self::Empty => None,

			Self::Unit { name, description } => Some(OpenapiSchema::new(Schema {
				schema_data: SchemaData {
					nullable: true,
					title: name,
					description,
					..Default::default()
				},
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

			Self::Option(opt) => opt
				.into_schema()
				.map(|mut schema| match schema.schema.schema_data.title.as_deref() {
					Some(_) => {
						let mut dependencies = IndexMap::new();
						let reference = inline_if_unnamed(&mut dependencies, schema, None);
						OpenapiSchema {
							schema: Schema {
								schema_data: SchemaData {
									nullable: true,
									..Default::default()
								},
								schema_kind: SchemaKind::AllOf { all_of: vec![reference] }
							},
							dependencies
						}
					},
					None => {
						schema.schema.schema_data.nullable = true;
						schema
					}
				}),

			Self::Enum {
				name,
				description,
				variants
			} => Some(OpenapiSchema::new(Schema {
				schema_data: SchemaData {
					title: name,
					description,
					..Default::default()
				},
				schema_kind: SchemaKind::Type(Type::String(StringType {
					enumeration: variants,
					..Default::default()
				}))
			})),

			Self::Array {
				items,
				len,
				unique_items
			} => {
				let mut dependencies = IndexMap::new();
				Some(OpenapiSchema {
					schema: Schema {
						schema_data: Default::default(),
						schema_kind: SchemaKind::Type(Type::Array(ArrayType {
							items: items
								.into_schema()
								.map(|schema| inline_if_unnamed(&mut dependencies, schema, None).boxed()),
							min_items: len,
							max_items: len,
							unique_items
						}))
					},
					dependencies
				})
			},

			Self::Object(obj) => Some(obj.into_schema()),

			Self::Alternatives(alt) => Some(alt.into_schema())
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
	type ArrayVisitor = Self;
	type ObjectVisitor = Object;
	type AlternativesVisitor = Alternatives;

	fn visit_unit_struct(&mut self, name: Option<String>, description: Option<String>) {
		self.panic_if_non_empty();
		*self = Self::Unit { name, description };
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

	fn visit_enum<I>(&mut self, name: Option<String>, description: Option<String>, variants: I)
	where
		I: IntoIterator<Item = String>
	{
		self.panic_if_non_empty();
		*self = Self::Enum {
			name,
			description,
			variants: variants.into_iter().map(|variant| Some((*variant).into())).collect()
		};
	}

	fn visit_array(&mut self, len: Option<usize>, unique_items: bool) -> &mut Self {
		self.panic_if_non_empty();
		*self = Self::Array {
			items: Box::new(Self::new()),
			len,
			unique_items
		};
		match self {
			Self::Array { items, .. } => items,
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

	fn visit_alternatives(&mut self) -> &mut Alternatives {
		self.panic_if_non_empty();
		*self = Self::Alternatives(Alternatives::default());
		match self {
			Self::Alternatives(alt) => alt,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
pub struct Field {
	doc: Option<String>,
	visitor: OpenapiVisitor
}

#[derive(Debug)]
enum Additional {
	/// Deny additional properties (serde: deny_unknown_fields)
	Deny,

	/// Allow additional properties (default)
	Allow,

	/// Allow specific additional property values
	Schema(Box<OpenapiVisitor>)
}

impl Default for Additional {
	fn default() -> Self {
		Self::Allow
	}
}

#[derive(Debug, Default)]
pub struct Object {
	name: Option<String>,
	description: Option<String>,
	fields: IndexMap<String, Field>,
	flatten_fields: Vec<Flatten>,
	additional: Additional
}

impl Object {
	fn fields(mut fields: IndexMap<String, Field>, flatten_fields: Vec<Flatten>) -> IndexMap<String, Field> {
		for obj in flatten_fields.into_iter().map(|f| f.obj) {
			if matches!(obj.additional, Additional::Schema(_)) {
				unimplemented!("flatten for maps is not currently implemented");
			}

			for (name, schema) in Self::fields(obj.fields, obj.flatten_fields) {
				if fields.contains_key(&name) {
					panic!("flatten produced multiple fields with name {name:?}");
				}
				let old_value = fields.insert(name, schema);
				debug_assert!(old_value.is_none());
			}
		}
		fields
	}

	fn into_schema(self) -> OpenapiSchema {
		let mut properties = IndexMap::new();
		let mut required = Vec::new();
		let mut dependencies = IndexMap::new();

		for (field_name, field) in Self::fields(self.fields, self.flatten_fields) {
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

		let additional_properties = match self.additional {
			Additional::Deny => Some(AdditionalProperties::Any(false)),
			Additional::Allow => None,
			Additional::Schema(schema) => schema
				.into_schema()
				.map(|schema| AdditionalProperties::Schema(Box::new(inline_if_unnamed(&mut dependencies, schema, None))))
		};

		let schema = Schema {
			schema_data: SchemaData {
				title: self.name,
				description: self.description,
				..Default::default()
			},
			schema_kind: SchemaKind::Type(Type::Object(ObjectType {
				properties,
				required,
				additional_properties,
				..Default::default()
			}))
		};
		OpenapiSchema { schema, dependencies }
	}
}

impl seal::Sealed for Object {}

impl ObjectVisitor for Object {
	type FieldVisitor = OpenapiVisitor;
	type FlattenVisitor = Flatten;
	type ValueVisitor = OpenapiVisitor;

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

	fn visit_flatten_field(&mut self) -> &mut Flatten {
		self.flatten_fields.push(Flatten::default());
		self.flatten_fields.last_mut().unwrap_or_else(|| unreachable!())
	}

	fn visit_deny_additional(&mut self) {
		if !matches!(self.additional, Additional::Allow) {
			panic!(
				"visit_additional or visit_deny_additional has been called before. You may only call this once per visitor."
			);
		}
		self.additional = Additional::Deny;
	}

	fn visit_additional(&mut self) -> &mut OpenapiVisitor {
		if !matches!(self.additional, Additional::Allow) {
			panic!(
				"visit_additional or visit_deny_additional has been called before. You may only call this once per visitor."
			);
		}
		self.additional = Additional::Schema(Box::new(OpenapiVisitor::new()));
		match self.additional {
			Additional::Schema(ref mut schema) => schema,
			_ => unreachable!()
		}
	}
}

#[derive(Debug, Default)]
pub struct Flatten {
	obj: Object
}

impl Flatten {
	#[track_caller]
	fn panic<T: Display>(&self, got: T) -> ! {
		panic!("can only flatten structs and maps (got {got})")
	}
}

impl seal::Sealed for Flatten {}

impl Visitor for Flatten {
	type OptionVisitor = Never;
	type ArrayVisitor = Never;
	type ObjectVisitor = Object;
	type AlternativesVisitor = Never;

	fn visit_unit(&mut self) {
		self.panic("a unit")
	}

	fn visit_unit_struct(&mut self, _name: Option<String>, _description: Option<String>) {
		self.panic("a unit struct")
	}

	fn visit_any(&mut self) {
		self.panic("any")
	}

	fn visit_bool(&mut self) {
		self.panic("a boolean")
	}

	fn visit_int(&mut self, _byte: Option<u32>, _minimum: Option<i64>) {
		self.panic("an integer")
	}

	fn visit_number(&mut self, _byte: Option<u32>) {
		self.panic("a number")
	}

	fn visit_char(&mut self) {
		self.panic("a char")
	}

	fn visit_string(&mut self) {
		self.panic("a string")
	}

	fn visit_uuid(&mut self) {
		self.panic("a uuid")
	}

	fn visit_date(&mut self) {
		self.panic("a date")
	}

	fn visit_datetime(&mut self) {
		self.panic("a datetime")
	}

	fn visit_option(&mut self) -> &mut Never {
		self.panic("an option")
	}

	fn visit_enum<I>(&mut self, _name: Option<String>, _description: Option<String>, _variants: I)
	where
		I: IntoIterator<Item = String>
	{
		self.panic("an enum")
	}

	fn visit_array(&mut self, _len: Option<usize>, _unique_items: bool) -> &mut Never {
		self.panic("an array")
	}

	fn visit_object(&mut self) -> &mut Object {
		&mut self.obj
	}

	fn visit_alternatives(&mut self) -> &mut Never {
		self.panic("alternatives")
	}
}

#[derive(Debug, Default)]
pub struct Alternatives {
	name: Option<String>,
	description: Option<String>,
	alts: Vec<OpenapiVisitor>
}

impl Alternatives {
	fn into_schema(self) -> OpenapiSchema {
		let mut dependencies = IndexMap::new();
		OpenapiSchema {
			schema: Schema {
				schema_data: SchemaData {
					title: self.name,
					description: self.description,
					..Default::default()
				},
				schema_kind: SchemaKind::OneOf {
					one_of: self
						.alts
						.into_iter()
						.filter_map(|ty| {
							ty.into_schema()
								.map(|schema| inline_if_unnamed(&mut dependencies, schema, None))
						})
						.collect()
				}
			},
			dependencies
		}
	}
}

impl seal::Sealed for Alternatives {}

impl AlternativesVisitor for Alternatives {
	type Visitor = OpenapiVisitor;

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

	fn visit_alternative(&mut self) -> &mut OpenapiVisitor {
		self.alts.push(OpenapiVisitor::new());
		self.alts.last_mut().unwrap_or_else(|| unreachable!())
	}
}
