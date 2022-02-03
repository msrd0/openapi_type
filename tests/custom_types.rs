#![allow(dead_code)]
use openapi_type::OpenapiType;

include!("util/test_type.rs");

#[derive(OpenapiType)]
struct UnitStruct;
test_type!(UnitStruct = {
	"nullable": true,
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
	"required": ["Success"],
	"additionalProperties": false
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
		"$ref": "#/components/schemas/EnumWithFields__Success__ExtTagWrapper"
	}, {
		"$ref": "#/components/schemas/EnumWithFields__Error__ExtTagWrapper"
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
	"EnumWithFields__Success__ExtTagWrapper": {
		"title": "EnumWithFields::Success::ExtTagWrapper",
		"type": "object",
		"properties": {
			"Success": {
				"$ref": "#/components/schemas/EnumWithFields__Success"
			}
		},
		"required": ["Success"],
		"additionalProperties": false
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
	},
	"EnumWithFields__Error__ExtTagWrapper": {
		"title": "EnumWithFields::Error::ExtTagWrapper",
		"type": "object",
		"properties": {
			"Error": {
				"$ref": "#/components/schemas/EnumWithFields__Error"
			}
		},
		"required": ["Error"],
		"additionalProperties": false
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
		"$ref": "#/components/schemas/EnumExternallyTagged__Success__ExtTagWrapper"
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
	},
	"EnumExternallyTagged__Success__ExtTagWrapper": {
		"title": "EnumExternallyTagged::Success::ExtTagWrapper",
		"type": "object",
		"properties": {
			"Success": {
				"$ref": "#/components/schemas/EnumExternallyTagged__Success"
			}
		},
		"required": ["Success"],
		"additionalProperties": false
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
		"$ref": "#/components/schemas/EnumInternallyTagged__Success"
	}, {
		"type": "object",
		"properties": {
			"ty": {
				"type": "string",
				"enum": ["Empty", "Error"]
			}
		},
		"required": ["ty"],
		"additionalProperties": false
	}]
}, {
	"EnumInternallyTagged__Success": {
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
	}
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
		"$ref": "#/components/schemas/EnumAdjacentlyTagged__Success__AdjTagWrapper"
	}, {
		"type": "object",
		"properties": {
			"ty": {
				"type": "string",
				"enum": ["Empty", "Error"]
			}
		},
		"required": ["ty"],
		"additionalProperties": false
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
	},
	"EnumAdjacentlyTagged__Success__AdjTagWrapper": {
		"title": "EnumAdjacentlyTagged::Success::AdjTagWrapper",
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
		"required": ["ty", "ct"],
		"additionalProperties": false
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
		"$ref": "#/components/schemas/EnumUntagged__Success"
	}, {
		"type": "object",
		"nullable": true,
		"additionalProperties": false
	}]
}, {
	"EnumUntagged__Success": {
		"title": "EnumUntagged::Success",
		"type": "object",
		"properties": {
			"value": {
				"type": "integer"
			}
		},
		"required": ["value"]
	}
});
