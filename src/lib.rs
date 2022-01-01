#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::tabs_in_doc_comments))]
#![doc = r##"
This crate gives static type information for primitives and commonly used types from the standard
library and other commonly used libraries `chrono`, `indexmap`, `linked-hash-map`, `time` and
`uuid` when the according feature is enabled. Also, it provides a derive macro for structs and
enums to gain access to their static type information at runtime.

The core of this crate is the [`OpenapiType`] trait. It has one static function,
[`schema`](OpenapiType::schema), which returns an [`OpenapiSchema`]. This assembles the static
type information in a way that is convenient to use for a generated OpenAPI specification, but
can also be utilized in other use cases as well.

# Custom Types
To gain access to the static type information of your custom types at runtime, the easiest way
is to use the derive macro:

```rust
# use openapi_type::OpenapiType;
#[derive(OpenapiType)]
struct FooBar {
	foo: String,
	bar: u64
}
# let schema = FooBar::schema().into_schema();
# let schema_json = serde_json::to_value(&schema).unwrap();
# assert_eq!(schema_json, serde_json::json!({
#   "type": "object",
#   "title": "FooBar",
#   "properties": {
#     "foo": {
#       "type": "string"
#     },
#     "bar": {
#       "type": "integer",
#       "format": "int64",
#       "minimum": 0
#     }
#   },
#   "required": ["foo", "bar"]
# }));
```

# OpenAPI specification
Using above type, running `FooBar::schema().into_schema()` yields

```yaml
type: object
title: FooBar
properties:
  foo:
    type: string
  bar:
    type: integer
    format: int64
    minimum: 0
required:
  - foo
  - bar
```

Note, however, that this is not sufficient for more complex types. If one of your structs fields
is a type that has a name (that is, `Type::schema().name` is not `None`), above schema will contain
a reference to that schema. Therefore, always remember to put the
[`dependencies`](OpenapiSchema::dependencies) into the specification alongside the type you are
interested in.
"##]

pub use indexmap;
pub use openapi_type_derive::OpenapiType;
pub use openapiv3 as openapi;

mod impls;
#[doc(hidden)]
pub mod private;

use indexmap::IndexMap;
use openapi::{Schema, SchemaData, SchemaKind};

/// This struct is used to generate the OpenAPI specification for a particular type. It is already
/// made available for all primitives and some other types from the rust standard library, and
/// you can also make your own types provide one through the [OpenapiType] trait and derive macro.
///
/// Note that this struct is marked non-exhaustive. This means that new attributes might be added
/// at any point in time without a breaking change. The only way to obtain a value is through the
/// [OpenapiSchema::new] method.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct OpenapiSchema {
	/// The name of this schema. If it is None, the schema will be inlined.
	pub name: Option<String>,
	/// The description of this schema. Optional and only makes sense when a [`name`](Self::name)
	/// is set as well.
	pub description: Option<String>,
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
			description: None,
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
				description: self.description,
				..Default::default()
			},
			schema_kind: self.schema
		}
	}
}

/// This trait needs to be implemented by every type that is being used in the OpenAPI Spec. It gives
/// access to the [OpenapiSchema] of this type. It is provided for primitive types, String and the
/// like. For use on your own types, there is a derive macro:
///
/// ```
/// # #[macro_use] extern crate openapi_type_derive;
/// #
/// #[derive(OpenapiType)]
/// struct MyResponse {
/// 	message: String
/// }
/// ```
pub trait OpenapiType {
	fn schema() -> OpenapiSchema;
}

impl<'a, T: ?Sized + OpenapiType> OpenapiType for &'a T {
	fn schema() -> OpenapiSchema {
		T::schema()
	}
}
