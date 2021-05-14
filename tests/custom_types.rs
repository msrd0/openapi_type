#![allow(dead_code)]
use openapi_type::OpenapiType;

include!("util/test_type.rs");

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
			"$ref": "#/components/schemas/EnumWithOneField__Success"
		}
	},
	"required": ["Success"]
}, {
	"EnumWithOneField__Success": {
		"title": "EnumWithOneField::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			}
		},
		"required": ["value"]
	}
});

#[derive(OpenapiType)]
enum EnumWithFields {
	Success { value: isize },
	Error { msg: String }
}
test_type!(EnumWithFields = {
	"title": "EnumWithFields",
	"oneOf": [{
		"type": "object",
		"properties": {
			"Success": {
				"$ref": "#/components/schemas/EnumWithFields__Success"
			}
		},
		"required": ["Success"]
	}, {
		"type": "object",
		"properties": {
			"Error": {
				"$ref": "#/components/schemas/EnumWithFields__Error"
			}
		},
		"required": ["Error"]
	}]
}, {
	"EnumWithFields__Success": {
		"title": "EnumWithFields::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			}
		},
		"required": ["value"]
	},
	"EnumWithFields__Error": {
		"title": "EnumWithFields::Error",
		"type": "object",
		"properties": {
			"msg": {
				"type": "string"
			}
		},
		"required": ["msg"]
	}
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
				"$ref": "#/components/schemas/EnumExternallyTagged__Success"
			}
		},
		"required": ["Success"]
	}, {
		"type": "string",
		"enum": ["Empty", "Error"]
	}]
}, {
	"EnumExternallyTagged__Success": {
		"title": "EnumExternallyTagged::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			}
		},
		"required": ["value"]
	}
});

#[derive(OpenapiType)]
#[openapi(tag = "ty")]
enum EnumInternallyTagged {
	Success { value: isize },
	Empty,
	Error
}
// TODO the Success variant should probably be $ref-ed
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
				"$ref": "#/components/schemas/EnumAdjacentlyTagged__Success"
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
}, {
	"EnumAdjacentlyTagged__Success": {
		"title": "EnumAdjacentlyTagged::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			}
		},
		"required": ["value"]
	}
});

#[derive(OpenapiType)]
#[openapi(untagged)]
enum EnumUntagged {
	Success { value: isize },
	Empty,
	Error
}
// TODO the Success variant should probably be $ref-ed
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
