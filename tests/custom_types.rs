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

#[derive(OpenapiType)]
struct UnitStruct;
test_type!(UnitStruct = {
	"type": "object",
	"title": "UnitStruct",
	"additionalProperties": false
});

#[derive(OpenapiType)]
struct SimpleStruct {
	foo: String,
	bar: isize
}
test_type!(SimpleStruct = {
	"type": "object",
	"title": "SimpleStruct",
	"properties": {
		"foo": {
			"type": "string"
		},
		"bar": {
			"type": "integer"
		}
	},
	"required": ["foo", "bar"]
});

#[derive(OpenapiType)]
#[openapi(rename = "FooBar")]
struct StructRename;
test_type!(StructRename = {
	"type": "object",
	"title": "FooBar",
	"additionalProperties": false
});

#[derive(OpenapiType)]
enum EnumWithoutFields {
	Success,
	Error
}
test_type!(EnumWithoutFields = {
	"type": "string",
	"title": "EnumWithoutFields",
	"enum": [
		"Success",
		"Error"
	]
});

#[derive(OpenapiType)]
enum EnumWithOneField {
	Success { value: isize }
}
test_type!(EnumWithOneField = {
	"type": "object",
	"title": "EnumWithOneField",
	"properties": {
		"Success": {
			"type": "object",
			"properties": {
				"value": {
					"type": "integer"
				}
			},
			"required": ["value"]
		}
	},
	"required": ["Success"]
});

#[derive(OpenapiType)]
enum EnumWithFields {
	Success { value: isize },
	Error { msg: String }
}
test_type!(EnumWithFields = {
	"title": "EnumWithFields",
	"oneOf": [{
		"title": "EnumWithFields::Success",
		"type": "object",
		"properties": {
			"Success": {
				"type": "object",
				"properties": {
					"value": {
						"type": "integer"
					}
				},
				"required": ["value"]
			}
		},
		"required": ["Success"]
	}, {
		"title": "EnumWithFields::Error",
		"type": "object",
		"properties": {
			"Error": {
				"type": "object",
				"properties": {
					"msg": {
						"type": "string"
					}
				},
				"required": ["msg"]
			}
		},
		"required": ["Error"]
	}]
});

#[derive(OpenapiType)]
enum EnumExternallyTagged {
	Success { value: isize },
	Empty,
	Error
}
test_type!(EnumExternallyTagged = {
	"title": "EnumExternallyTagged",
	"oneOf": [{
		"type": "object",
		"properties": {
			"Success": {
				"title": "EnumExternallyTagged::Success",
				"type": "object",
				"properties": {
					"value": {
						"type": "integer"
					}
				},
				"required": ["value"]
			}
		},
		"required": ["Success"]
	}, {
		"type": "string",
		"enum": ["Empty", "Error"]
	}]
});

#[derive(OpenapiType)]
#[openapi(tag = "ty")]
enum EnumInternallyTagged {
	Success { value: isize },
	Empty,
	Error
}
test_type!(EnumInternallyTagged = {
	"title": "EnumInternallyTagged",
	"oneOf": [{
		"title": "EnumInternallyTagged::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			},
			"ty": {
				"type": "string",
				"enum": ["Success"]
			}
		},
		"required": ["value", "ty"]
	}, {
		"type": "object",
		"properties": {
			"ty": {
				"type": "string",
				"enum": ["Empty", "Error"]
			}
		},
		"required": ["ty"]
	}]
});

#[derive(OpenapiType)]
#[openapi(tag = "ty", content = "ct")]
enum EnumAdjacentlyTagged {
	Success { value: isize },
	Empty,
	Error
}
test_type!(EnumAdjacentlyTagged = {
	"title": "EnumAdjacentlyTagged",
	"oneOf": [{
		"type": "object",
		"properties": {
			"ty": {
				"type": "string",
				"enum": ["Success"]
			},
			"ct": {
				"title": "EnumAdjacentlyTagged::Success",
				"type": "object",
				"properties": {
					"value": {
						"type": "integer"
					}
				},
				"required": ["value"]
			}
		},
		"required": ["ty", "ct"]
	}, {
		"type": "object",
		"properties": {
			"ty": {
				"type": "string",
				"enum": ["Empty", "Error"]
			}
		},
		"required": ["ty"]
	}]
});

#[derive(OpenapiType)]
#[openapi(untagged)]
enum EnumUntagged {
	Success { value: isize },
	Empty,
	Error
}
test_type!(EnumUntagged = {
	"title": "EnumUntagged",
	"oneOf": [{
		"title": "EnumUntagged::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			}
		},
		"required": ["value"]
	}, {
		"type": "object",
		"additionalProperties": false
	}]
});
