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
				assert_eq!(schema_json, expected);
			}
		}
	};
}

#[derive(OpenapiType)]
#[openapi(rename = "FooBar")]
struct ContainerRename;
test_type!(ContainerRename = {
	"type": "object",
	"title": "FooBar",
	"additionalProperties": false
});

macro_rules! test_rename_all {
	(rename_all = $rename_all:literal, foo_bar = $foo_bar:literal, faa_bor = $faa_bor:literal) => {
		paste::paste! {
			#[derive(OpenapiType, serde::Serialize)]
			#[openapi(rename = "FooBar")]
			#[serde(rename_all = $rename_all)]
			#[allow(non_camel_case_types, non_snake_case)]
			struct [< ContainerRenameAll_ $rename_all >] {
				foo_bar: Option<&'static str>,
				FaaBor: Option<&'static str>
			}
			test_type!([< ContainerRenameAll_ $rename_all >] = {
				"type": "object",
				"title": "FooBar",
				"properties": {
					$foo_bar: {
						"type": "string"
					},
					$faa_bor: {
						"type": "string"
					}
				}
			});
			#[test]
			fn [< containerrenameall_ $rename_all:lower _serde >]() {
				let value = [< ContainerRenameAll_ $rename_all >] {
					foo_bar: Some("foo_bar"),
					FaaBor: Some("faa_bor")
				};
				let _json = serde_json::to_value(&value).unwrap();
				let _expected = serde_json::json!({
					$foo_bar: "foo_bar",
					$faa_bor: "faa_bor"
				});
				// assert_eq!(json, expected);
				// TODO serde does not do this conversion properly but rather assumes
				// that all fields are snake_case and all variants are TitleCase to begin
				// with
			}
		}
	};
}

test_rename_all!(rename_all = "lowercase", foo_bar = "foo_bar", faa_bor = "faabor");
test_rename_all!(rename_all = "UPPERCASE", foo_bar = "FOO_BAR", faa_bor = "FAABOR");
test_rename_all!(rename_all = "PascalCase", foo_bar = "FooBar", faa_bor = "FaaBor");
test_rename_all!(rename_all = "camelCase", foo_bar = "fooBar", faa_bor = "faaBor");
test_rename_all!(rename_all = "snake_case", foo_bar = "foo_bar", faa_bor = "faa_bor");
test_rename_all!(rename_all = "SCREAMING_SNAKE_CASE", foo_bar = "FOO_BAR", faa_bor = "FAA_BOR");
test_rename_all!(rename_all = "kebab-case", foo_bar = "foo-bar", faa_bor = "faa-bor");
test_rename_all!(rename_all = "SCREAMING-KEBAB-CASE", foo_bar = "FOO-BAR", faa_bor = "FAA-BOR");
