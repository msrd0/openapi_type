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
mod visitor;

pub use visitor::{ObjectVisitor, OpenapiSchema, OpenapiVisitor, Visitor};

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
	fn visit_type<V: Visitor>(visitor: &mut V);

	fn schema() -> OpenapiSchema {
		let mut visitor = OpenapiVisitor::new();
		Self::visit_type(&mut visitor);
		visitor
			.into_schema()
			.expect("The OpenapiType implementation failed to call the visitor")
	}
}

impl<'a, T: ?Sized + OpenapiType> OpenapiType for &'a T {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		T::visit_type(visitor)
	}
}
