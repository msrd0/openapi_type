#[cfg(feature = "chrono")]
use chrono::{Date, DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, Utc};
use indexmap::{IndexMap, IndexSet};
use openapi_type::OpenapiType;
use serde_json::Value;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize}
};
#[cfg(feature = "uuid")]
use uuid::Uuid;

macro_rules! test_type {
	($($ty:ident $(<$($generic:ident),+>)*),* = $json:tt) => {
		paste::paste! { $(
			#[test]
			fn [< $ty:lower $($(_ $generic:lower)+)* >]() {
				let schema = <$ty $(<$($generic),+>)* as OpenapiType>::schema();
				let schema = openapi_type::OpenapiSchema::into_schema(schema);
				let schema_json = serde_json::to_value(&schema).unwrap();
				let expected = serde_json::json!($json);
				pretty_assertions::assert_eq!(schema_json, expected);
			}
		)* }
	};
}

type Unit = ();
test_type!(Unit = {
	"type": "object",
	"additionalProperties": false
});

test_type!(Value = {
	"nullable": true
});

test_type!(bool = {
	"type": "boolean"
});

// ### integer types

test_type!(isize = {
	"type": "integer"
});

test_type!(usize = {
	"type": "integer",
	"minimum": 0
});

test_type!(i8 = {
	"type": "integer",
	"format": "int8"
});

test_type!(u8 = {
	"type": "integer",
	"format": "int8",
	"minimum": 0
});

test_type!(i16 = {
	"type": "integer",
	"format": "int16"
});

test_type!(u16 = {
	"type": "integer",
	"format": "int16",
	"minimum": 0
});

test_type!(i32 = {
	"type": "integer",
	"format": "int32"
});

test_type!(u32 = {
	"type": "integer",
	"format": "int32",
	"minimum": 0
});

test_type!(i64 = {
	"type": "integer",
	"format": "int64"
});

test_type!(u64 = {
	"type": "integer",
	"format": "int64",
	"minimum": 0
});

test_type!(i128 = {
	"type": "integer",
	"format": "int128"
});

test_type!(u128 = {
	"type": "integer",
	"format": "int128",
	"minimum": 0
});

// ### non-zero integer types

test_type!(NonZeroUsize = {
	"type": "integer",
	"minimum": 1
});

test_type!(NonZeroU8 = {
	"type": "integer",
	"format": "int8",
	"minimum": 1
});

test_type!(NonZeroU16 = {
	"type": "integer",
	"format": "int16",
	"minimum": 1
});

test_type!(NonZeroU32 = {
	"type": "integer",
	"format": "int32",
	"minimum": 1
});

test_type!(NonZeroU64 = {
	"type": "integer",
	"format": "int64",
	"minimum": 1
});

test_type!(NonZeroU128 = {
	"type": "integer",
	"format": "int128",
	"minimum": 1
});

// ### floats

test_type!(f32 = {
	"type": "number",
	"format": "float"
});

test_type!(f64 = {
	"type": "number",
	"format": "double"
});

// ### string

test_type!(String = {
	"type": "string"
});

#[cfg(feature = "uuid")]
test_type!(Uuid = {
	"type": "string",
	"format": "uuid"
});

// ### date/time

#[cfg(feature = "chrono")]
test_type!(Date<FixedOffset>, Date<Local>, Date<Utc>, NaiveDate = {
	"type": "string",
	"format": "date"
});

#[cfg(feature = "chrono")]
test_type!(DateTime<FixedOffset>, DateTime<Local>, DateTime<Utc>, NaiveDateTime = {
	"type": "string",
	"format": "date-time"
});

// ### some std types

test_type!(Option<String> = {
	"type": "string",
	"nullable": true
});

test_type!(Vec<String> = {
	"type": "array",
	"items": {
		"type": "string"
	}
});

test_type!(BTreeSet<String>, IndexSet<String>, HashSet<String> = {
	"type": "array",
	"items": {
		"type": "string"
	},
	"uniqueItems": true
});

test_type!(BTreeMap<isize, String>, IndexMap<isize, String>, HashMap<isize, String> = {
	"type": "object",
	"properties": {
		"default": {
			"type": "integer"
		}
	},
	"required": ["default"],
	"additionalProperties": {
		"type": "string"
	}
});
