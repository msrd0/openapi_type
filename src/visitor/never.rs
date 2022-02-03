use super::{seal, AlternativesVisitor, ObjectVisitor, Visitor};

#[allow(missing_debug_implementations)]
pub struct Never(());

impl seal::Sealed for Never {}

impl Visitor for Never {
	type OptionVisitor = Self;
	type ArrayVisitor = Self;
	type ObjectVisitor = Self;
	type AlternativesVisitor = Self;

	fn visit_unit(&mut self) {
		unreachable!()
	}

	fn visit_unit_struct(&mut self, _name: Option<String>, _description: Option<String>) {
		unreachable!()
	}

	fn visit_any(&mut self) {
		unreachable!()
	}

	fn visit_bool(&mut self) {
		unreachable!()
	}

	fn visit_int(&mut self, _byte: Option<u32>, _minimum: Option<i64>) {
		unreachable!()
	}

	fn visit_number(&mut self, _byte: Option<u32>) {
		unreachable!()
	}

	fn visit_char(&mut self) {
		unreachable!()
	}

	fn visit_string(&mut self) {
		unreachable!()
	}

	fn visit_uuid(&mut self) {
		unreachable!()
	}

	fn visit_date(&mut self) {
		unreachable!()
	}

	fn visit_datetime(&mut self) {
		unreachable!()
	}

	fn visit_option(&mut self) -> &mut Self {
		unreachable!()
	}

	fn visit_enum<I>(&mut self, _name: Option<String>, _description: Option<String>, _variants: I)
	where
		I: IntoIterator<Item = String>
	{
		unreachable!()
	}

	fn visit_array(&mut self, _len: Option<usize>, _unique_items: bool) -> &mut Self {
		unreachable!()
	}

	fn visit_object(&mut self) -> &mut Self {
		unreachable!()
	}

	fn visit_alternatives(&mut self) -> &mut Self {
		unreachable!()
	}
}

impl ObjectVisitor for Never {
	type FieldVisitor = Self;
	type FlattenVisitor = Self;
	type ValueVisitor = Self;

	fn visit_name(&mut self, _name: String) {
		unreachable!()
	}

	fn visit_description(&mut self, _description: String) {
		unreachable!()
	}

	fn visit_field(&mut self, _name: String, _doc: Option<String>) -> &mut Self {
		unreachable!()
	}

	fn visit_flatten_field(&mut self) -> &mut Self {
		unreachable!()
	}

	fn visit_deny_additional(&mut self) {
		unreachable!()
	}

	fn visit_additional(&mut self) -> &mut Self {
		unreachable!()
	}
}

impl AlternativesVisitor for Never {
	type Visitor = Self;

	fn visit_name(&mut self, _name: String) {
		unreachable!()
	}

	fn visit_description(&mut self, _description: String) {
		unreachable!()
	}

	fn visit_alternative(&mut self) -> &mut Self {
		unreachable!()
	}
}
