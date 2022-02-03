mod never;
mod openapi;

pub use openapi::{OpenapiSchema, OpenapiVisitor};

mod seal {
	pub trait Sealed {}
}

/// This trait can be used to visit a type. Call **one** of the methods on this
/// trait **exactly once**.
pub trait Visitor: seal::Sealed {
	type OptionVisitor: Visitor;
	type ArrayVisitor: Visitor;
	type ObjectVisitor: ObjectVisitor;
	type AlternativesVisitor: AlternativesVisitor;

	fn visit_unit(&mut self) {
		self.visit_unit_struct(None, None);
	}

	fn visit_unit_struct(&mut self, name: Option<String>, description: Option<String>);

	fn visit_any(&mut self);

	fn visit_bool(&mut self);

	fn visit_int(&mut self, byte: Option<u32>, minimum: Option<i64>);

	fn visit_number(&mut self, byte: Option<u32>);

	fn visit_char(&mut self);

	fn visit_string(&mut self);

	fn visit_uuid(&mut self);

	fn visit_date(&mut self);

	fn visit_datetime(&mut self);

	fn visit_option(&mut self) -> &mut Self::OptionVisitor;

	fn visit_enum<I>(&mut self, name: Option<String>, description: Option<String>, variants: I)
	where
		I: IntoIterator<Item = String>;

	fn visit_array(&mut self, len: Option<usize>, unique_items: bool) -> &mut Self::ArrayVisitor;

	fn visit_object(&mut self) -> &mut Self::ObjectVisitor;

	fn visit_alternatives(&mut self) -> &mut Self::AlternativesVisitor;
}

pub trait ObjectVisitor: seal::Sealed {
	type FieldVisitor: Visitor;
	type FlattenVisitor: Visitor;
	type ValueVisitor: Visitor;

	fn visit_name(&mut self, name: String);

	fn visit_description(&mut self, description: String);

	fn visit_field(&mut self, name: String, doc: Option<String>) -> &mut Self::FieldVisitor;

	fn visit_flatten_field(&mut self) -> &mut Self::FlattenVisitor;

	fn visit_deny_additional(&mut self);

	fn visit_additional(&mut self) -> &mut Self::ValueVisitor;
}

pub trait AlternativesVisitor: seal::Sealed {
	type Visitor: Visitor;

	fn visit_name(&mut self, name: String);

	fn visit_description(&mut self, description: String);

	fn visit_alternative(&mut self) -> &mut Self::Visitor;
}
