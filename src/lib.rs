#![warn(missing_debug_implementations, rust_2018_idioms)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::tabs_in_doc_comments))]
/*!
TODO
*/

pub use indexmap;
pub use openapi_type_derive::OpenapiType;
pub use openapiv3 as openapi;

mod impls;
#[doc(hidden)]
pub mod private;

use indexmap::IndexMap;
use openapi::{Schema, SchemaData, SchemaKind};

// TODO update the documentation
/**
This struct needs to be available for every type that can be part of an OpenAPI Spec. It is
already implemented for primitive types, String, Vec, Option and the like. To have it available
for your type, simply derive from [OpenapiType].
*/
#[derive(Debug, Clone, PartialEq)]
pub struct OpenapiSchema {
	/// The name of this schema. If it is None, the schema will be inlined.
	pub name: Option<String>,
	/// Whether this particular schema is nullable. Note that there is no guarantee that this will
	/// make it into the final specification, it might just be interpreted as a hint to make it
	/// an optional parameter.
	pub nullable: bool,
	/// The actual OpenAPI schema.
	pub schema: SchemaKind,
	/// Other schemas that this schema depends on. They will be included in the final OpenAPI Spec
	/// along with this schema.
	pub dependencies: IndexMap<String, OpenapiSchema>
}

impl OpenapiSchema {
	/// Create a new schema that has no name.
	pub fn new(schema: SchemaKind) -> Self {
		Self {
			name: None,
			nullable: false,
			schema,
			dependencies: IndexMap::new()
		}
	}

	/// Convert this schema to a [Schema] that can be serialized to the OpenAPI Spec.
	pub fn into_schema(self) -> Schema {
		Schema {
			schema_data: SchemaData {
				nullable: self.nullable,
				title: self.name,
				..Default::default()
			},
			schema_kind: self.schema
		}
	}
}

/**
This trait needs to be implemented by every type that is being used in the OpenAPI Spec. It gives
access to the [OpenapiSchema] of this type. It is provided for primitive types, String and the
like. For use on your own types, there is a derive macro:

```
# #[macro_use] extern crate openapi_type_derive;
#
#[derive(OpenapiType)]
struct MyResponse {
	message: String
}
```
*/
pub trait OpenapiType {
	fn schema() -> OpenapiSchema;
}

impl<'a, T: ?Sized + OpenapiType> OpenapiType for &'a T {
	fn schema() -> OpenapiSchema {
		T::schema()
	}
}
