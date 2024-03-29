#![allow(dead_code)]
use openapi_type::OpenapiType;

include!("util/test_type.rs");

#[derive(OpenapiType)]
#[openapi(deny_unknown_fields)]
struct StructDenyUnknown {
	foo: String
}
test_type!(StructDenyUnknown = {
	"type": "object",
	"title": "StructDenyUnknown",
	"properties": {
		"foo": {
			"type": "string"
		}
	},
	"required": ["foo"],
	"additionalProperties": false
});

/// Very cool struct!
#[derive(OpenapiType)]
struct StructDoc {
	/// Very important!
	foo: String
}
test_type!(StructDoc = {
	"type": "object",
	"title": "StructDoc",
	"description": "Very cool struct!",
	"properties": {
		"foo": {
			"type": "string",
			"description": "Very important!"
		}
	},
	"required": ["foo"]
});

/// Very cool enum!
#[derive(OpenapiType)]
enum EnumDoc {
	/// Look, a message!
	Message {
		/// The text of the message in markdown format.
		text: String
	},
	/// Oops
	Error
}
// TODO the variant doc isn't currently included
test_type!(EnumDoc = {
	"title": "EnumDoc",
	"description": "Very cool enum!",
	"oneOf": [{
		"$ref": "#/components/schemas/EnumDoc__Message__ExtTagWrapper"
	}, {
		"type": "string",
		"enum": ["Error"]
	}]
}, {
	"EnumDoc__Message": {
		"title": "EnumDoc::Message",
		"type": "object",
		"properties": {
			"text": {
				"type": "string",
				"description": "The text of the message in markdown format."
			}
		},
		"required": ["text"]
	},
	"EnumDoc__Message__ExtTagWrapper": {
		"title": "EnumDoc::Message::ExtTagWrapper",
		"type": "object",
		"properties": {
			"Message": {
				"$ref": "#/components/schemas/EnumDoc__Message"
			}
		},
		"required": ["Message"],
		"additionalProperties": false
	}
});

#[derive(OpenapiType)]
#[openapi(rename = "FooBar")]
struct ContainerRename;
test_type!(ContainerRename = {
	"nullable": true,
	"type": "object",
	"title": "FooBar",
	"additionalProperties": false
});

#[derive(OpenapiType)]
#[openapi(rename_all = "UPPERCASE")]
struct FieldRename {
	#[openapi(rename = "bar")]
	foo: String
}
test_type!(FieldRename = {
	"type": "object",
	"title": "FieldRename",
	"properties": {
		"bar": {
			"type": "string"
		}
	},
	"required": ["bar"]
});

#[derive(OpenapiType)]
struct FieldFlattenInner {
	inner: String
}
#[derive(OpenapiType)]
struct FieldFlatten {
	outer: String,
	#[openapi(flatten)]
	flat: FieldFlattenInner
}
test_type!(FieldFlatten = {
	"type": "object",
	"title": "FieldFlatten",
	"properties": {
		"inner": {
			"type": "string"
		},
		"outer": {
			"type": "string"
		}
	},
	"required": ["outer", "inner"]
});

#[derive(OpenapiType)]
struct FieldSkip {
	#[openapi(skip_serializing, skip_deserializing)]
	foo: String,
	#[openapi(skip)]
	bar: String
}
test_type!(FieldSkip = {
	"type": "object",
	"title": "FieldSkip"
});

#[derive(OpenapiType)]
struct FieldNullable {
	#[openapi(skip_serializing)]
	foo0: String,
	#[openapi(skip_deserializing)]
	foo1: String,
	#[openapi(default)]
	foo2: String,
	#[openapi(default = "String::new")]
	foo3: String,
	#[openapi(skip_serializing_if = "String::is_empty")]
	foo4: String
}
test_type!(FieldNullable = {
	"type": "object",
	"title": "FieldNullable",
	"properties": {
		"foo0": {
			"type": "string"
		},
		"foo1": {
			"type": "string"
		},
		"foo2": {
			"type": "string"
		},
		"foo3": {
			"type": "string"
		},
		"foo4": {
			"type": "string"
		}
	}
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
