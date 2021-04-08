#![allow(dead_code)]
use openapi_type::OpenapiType;

macro_rules! test_type {
	($ty:ty = $json:tt) => {
		paste::paste! {
			#[test]
			fn [< $ty:lower >]() {
				let schema = <$ty as OpenapiType>::schema();
				let schema = openapi_type::OpenapiSchema::into_schema(schema);
				let schema_json = serde_json::to_value(&schema).unwrap();
				let expected = serde_json::json!($json);
				pretty_assertions::assert_eq!(schema_json, expected);
			}
		}
	};
}

/// Very cool struct!
#[derive(OpenapiType)]
struct ContainerDoc;
test_type!(ContainerDoc = {
	"type": "object",
	"title": "ContainerDoc",
	"description": "Very cool struct!",
	"additionalProperties": false
});

#[derive(OpenapiType)]
#[openapi(rename = "FooBar")]
struct ContainerRename;
test_type!(ContainerRename = {
	"type": "object",
	"title": "FooBar",
	"additionalProperties": false
});

macro_rules! test_rename_all {
	(rename_all = $rename_all:literal, foo_bar = $foo_bar:literal) => {
		paste::paste! {
			#[derive(OpenapiType, serde::Serialize)]
			#[openapi(rename = "FooBar")]
			#[serde(rename_all = $rename_all)]
			#[allow(non_camel_case_types, non_snake_case)]
			struct [< ContainerRenameAll_ $rename_all >] {
				foo_bar: Option<&'static str>,
			}
			test_type!([< ContainerRenameAll_ $rename_all >] = {
				"type": "object",
				"title": "FooBar",
				"properties": {
					$foo_bar: {
						"type": "string"
					},
				}
			});
			#[test]
			fn [< containerrenameall_ $rename_all:lower _serde >]() {
				let value = [< ContainerRenameAll_ $rename_all >] {
					foo_bar: Some("foo_bar"),
				};
				let json = serde_json::to_value(&value).unwrap();
				let expected = serde_json::json!({
					$foo_bar: "foo_bar",
				});
				pretty_assertions::assert_eq!(json, expected);
			}
		}
	};
}

test_rename_all!(rename_all = "lowercase", foo_bar = "foo_bar");
test_rename_all!(rename_all = "UPPERCASE", foo_bar = "FOO_BAR");
test_rename_all!(rename_all = "PascalCase", foo_bar = "FooBar");
test_rename_all!(rename_all = "camelCase", foo_bar = "fooBar");
test_rename_all!(rename_all = "snake_case", foo_bar = "foo_bar");
test_rename_all!(rename_all = "SCREAMING_SNAKE_CASE", foo_bar = "FOO_BAR");
test_rename_all!(rename_all = "kebab-case", foo_bar = "foo-bar");
test_rename_all!(rename_all = "SCREAMING-KEBAB-CASE", foo_bar = "FOO-BAR");
