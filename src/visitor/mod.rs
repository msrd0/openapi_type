mod openapi;
pub use openapi::OpenapiVisitor;

mod seal {
	pub trait Sealed {}
}

/// This trait can be used to visit a type. Call **one** of the methods on this
/// trait **exactly once**.
pub trait Visitor: seal::Sealed {
	type ObjectVisitor: ObjectVisitor;

	fn visit_object(&mut self) -> &mut Self::ObjectVisitor;
}

pub trait ObjectVisitor: seal::Sealed {
	type FieldVisitor: Visitor;

	fn visit_name(&mut self, name: String);

	fn visit_description(&mut self, description: String);

	fn visit_field(&mut self, name: String, doc: Option<String>) -> &mut Self::FieldVisitor;
}
