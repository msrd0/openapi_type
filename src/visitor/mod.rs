mod openapi;
pub use openapi::OpenapiVisitor;

pub trait Visitor {
	type ObjectVisitor: ObjectVisitor;
	
	fn visit_object(&mut self) -> &mut Self::ObjectVisitor;
}

pub trait ObjectVisitor {
	type FieldVisitor: Visitor;
	
	fn visit_name(&mut self, name: String);
	
	fn visit_description(&mut self, description: String);
	
	fn visit_field(&mut self, name: String) -> &mut Self::FieldVisitor;
}