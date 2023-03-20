use indexmap::{IndexMap, IndexSet};
use openapi_type::OpenapiType;
use serde_json::Value;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize}
};

macro_rules! test_type {
	($($($ty:ident)::+ $(<$($($generic:ident)::+),+>)?),* = $json:tt) => {
		paste::paste! {
			$(
				#[test]
				fn [<$($ty:lower)_+ $($($(_$generic:lower)+)+)? >]() {
					let schema = <$($ty)::+ $(<$($($generic)::+),+>)? as OpenapiType>::schema();
					let schema = &schema.schema;
					let schema_json = serde_json::to_value(&schema).unwrap();
					let expected = serde_json::json!($json);
					pretty_assertions::assert_eq!(schema_json, expected);
				}
			)*
		}
	};
}

type Unit = ();
test_type!(Unit = {
	"nullable": true,
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

#[cfg(feature = "uuid08")]
test_type!(uuid08::Uuid = {
	"type": "string",
	"format": "uuid"
});

#[cfg(feature = "uuid1")]
test_type!(uuid1::Uuid = {
	"type": "string",
	"format": "uuid"
});

// ### date/time

#[cfg(feature = "chrono04")]
test_type!(chrono04::Date<chrono04::FixedOffset>, chrono04::Date<chrono04::Utc>, chrono04::NaiveDate = {
	"type": "string",
	"format": "date"
});

#[cfg(feature = "time03")]
test_type!(time03::Date = {
	"type": "string",
	"format": "date"
});

#[cfg(feature = "chrono04")]
test_type!(chrono04::DateTime<chrono04::FixedOffset>, chrono04::DateTime<chrono04::Utc>, chrono04::NaiveDateTime = {
	"type": "string",
	"format": "date-time"
});

#[cfg(feature = "time03")]
test_type!(time03::OffsetDateTime, time03::PrimitiveDateTime = {
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

test_type!(BTreeMap<isize, String>, HashMap<isize, String>, IndexMap<isize, String> = {
	"type": "object",
	"additionalProperties": {
		"type": "string"
	}
});

#[cfg(feature = "linked-hash-map05")]
test_type!(linked_hash_map05::LinkedHashMap<isize, String> = {
	"type": "object",
	"additionalProperties": {
		"type": "string"
	}
});
