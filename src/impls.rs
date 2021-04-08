use crate::{OpenapiSchema, OpenapiType};
#[cfg(feature = "chrono")]
use chrono::{offset::TimeZone, Date, DateTime, NaiveDate, NaiveDateTime};
use indexmap::{IndexMap, IndexSet};
use openapiv3::{
	AdditionalProperties, ArrayType, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr, SchemaKind,
	StringFormat, StringType, Type, VariantOrUnknownOrEmpty
};
use serde_json::Value;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	hash::BuildHasher,
	num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize}
};
#[cfg(feature = "uuid")]
use uuid::Uuid;

macro_rules! impl_openapi_type {
	($($ty:ident $(<$($generic:ident : $bound:path),+>)*),* => $schema:expr) => {
		$(
			impl $(<$($generic : $bound),+>)* OpenapiType for $ty $(<$($generic),+>)* {
				fn schema() -> OpenapiSchema {
					$schema
				}
			}
		)*
	};
}

type Unit = ();
impl_openapi_type!(Unit => {
	OpenapiSchema::new(SchemaKind::Type(Type::Object(ObjectType {
		additional_properties: Some(AdditionalProperties::Any(false)),
		..Default::default()
	})))
});

impl_openapi_type!(Value => {
	OpenapiSchema {
		nullable: true,
		description: None,
		name: None,
		schema: SchemaKind::Any(Default::default()),
		dependencies: Default::default()
	}
});

impl_openapi_type!(bool => OpenapiSchema::new(SchemaKind::Type(Type::Boolean {})));

#[inline]
fn int_schema(minimum: Option<i64>, bits: Option<i64>) -> OpenapiSchema {
	OpenapiSchema::new(SchemaKind::Type(Type::Integer(IntegerType {
		minimum,
		format: bits
			.map(|bits| VariantOrUnknownOrEmpty::Unknown(format!("int{}", bits)))
			.unwrap_or(VariantOrUnknownOrEmpty::Empty),
		..Default::default()
	})))
}

impl_openapi_type!(isize => int_schema(None, None));
impl_openapi_type!(i8 => int_schema(None, Some(8)));
impl_openapi_type!(i16 => int_schema(None, Some(16)));
impl_openapi_type!(i32 => int_schema(None, Some(32)));
impl_openapi_type!(i64 => int_schema(None, Some(64)));
impl_openapi_type!(i128 => int_schema(None, Some(128)));

impl_openapi_type!(usize => int_schema(Some(0), None));
impl_openapi_type!(u8 => int_schema(Some(0), Some(8)));
impl_openapi_type!(u16 => int_schema(Some(0), Some(16)));
impl_openapi_type!(u32 => int_schema(Some(0), Some(32)));
impl_openapi_type!(u64 => int_schema(Some(0), Some(64)));
impl_openapi_type!(u128 => int_schema(Some(0), Some(128)));

impl_openapi_type!(NonZeroUsize => int_schema(Some(1), None));
impl_openapi_type!(NonZeroU8 => int_schema(Some(1), Some(8)));
impl_openapi_type!(NonZeroU16 => int_schema(Some(1), Some(16)));
impl_openapi_type!(NonZeroU32 => int_schema(Some(1), Some(32)));
impl_openapi_type!(NonZeroU64 => int_schema(Some(1), Some(64)));
impl_openapi_type!(NonZeroU128 => int_schema(Some(1), Some(128)));

#[inline]
fn float_schema(format: NumberFormat) -> OpenapiSchema {
	OpenapiSchema::new(SchemaKind::Type(Type::Number(NumberType {
		format: VariantOrUnknownOrEmpty::Item(format),
		..Default::default()
	})))
}

impl_openapi_type!(f32 => float_schema(NumberFormat::Float));
impl_openapi_type!(f64 => float_schema(NumberFormat::Double));

#[inline]
fn str_schema(format: VariantOrUnknownOrEmpty<StringFormat>) -> OpenapiSchema {
	OpenapiSchema::new(SchemaKind::Type(Type::String(StringType {
		format,
		..Default::default()
	})))
}

impl_openapi_type!(String, str => str_schema(VariantOrUnknownOrEmpty::Empty));

#[cfg(feature = "chrono")]
impl_openapi_type!(Date<T: TimeZone>, NaiveDate => {
	str_schema(VariantOrUnknownOrEmpty::Item(StringFormat::Date))
});

#[cfg(feature = "chrono")]
impl_openapi_type!(DateTime<T: TimeZone>, NaiveDateTime => {
	str_schema(VariantOrUnknownOrEmpty::Item(StringFormat::DateTime))
});

#[cfg(feature = "uuid")]
impl_openapi_type!(Uuid => {
	str_schema(VariantOrUnknownOrEmpty::Unknown("uuid".to_owned()))
});

impl_openapi_type!(Option<T: OpenapiType> => {
	let schema = T::schema();
	let mut dependencies = schema.dependencies.clone();
	let schema = match schema.name.clone() {
		Some(name) => {
			let reference = ReferenceOr::Reference {
				reference: format!("#/components/schemas/{}", name)
			};
			dependencies.insert(name, schema);
			SchemaKind::AllOf { all_of: vec![reference] }
		},
		None => schema.schema
	};

	OpenapiSchema {
		nullable: true,
		name: None,
		description: None,
		schema,
		dependencies
	}
});

#[inline]
fn array_schema<T: OpenapiType>(unique_items: bool) -> OpenapiSchema {
	let schema = T::schema();
	let mut dependencies = schema.dependencies.clone();

	let items = match schema.name.clone() {
		Some(name) => {
			let reference = ReferenceOr::Reference {
				reference: format!("#/components/schemas/{}", name)
			};
			dependencies.insert(name, schema);
			reference
		},
		None => ReferenceOr::Item(Box::new(schema.into_schema()))
	};

	OpenapiSchema {
		nullable: false,
		name: None,
		description: None,
		schema: SchemaKind::Type(Type::Array(ArrayType {
			items,
			min_items: None,
			max_items: None,
			unique_items
		})),
		dependencies
	}
}

impl_openapi_type!(Vec<T: OpenapiType> => array_schema::<T>(false));
impl_openapi_type!(BTreeSet<T: OpenapiType>, IndexSet<T: OpenapiType>, HashSet<T: OpenapiType, S: BuildHasher> => {
	array_schema::<T>(true)
});

#[inline]
fn map_schema<K: OpenapiType, T: OpenapiType>() -> OpenapiSchema {
	let key_schema = K::schema();
	let mut dependencies = key_schema.dependencies.clone();

	let keys = match key_schema.name.clone() {
		Some(name) => {
			let reference = ReferenceOr::Reference {
				reference: format!("#/components/schemas/{}", name)
			};
			dependencies.insert(name, key_schema);
			reference
		},
		None => ReferenceOr::Item(Box::new(key_schema.into_schema()))
	};

	let schema = T::schema();
	dependencies.extend(schema.dependencies.iter().map(|(k, v)| (k.clone(), v.clone())));

	let items = Box::new(match schema.name.clone() {
		Some(name) => {
			let reference = ReferenceOr::Reference {
				reference: format!("#/components/schemas/{}", name)
			};
			dependencies.insert(name, schema);
			reference
		},
		None => ReferenceOr::Item(schema.into_schema())
	});

	let mut properties = IndexMap::new();
	properties.insert("default".to_owned(), keys);

	OpenapiSchema {
		nullable: false,
		name: None,
		description: None,
		schema: SchemaKind::Type(Type::Object(ObjectType {
			properties,
			required: vec!["default".to_owned()],
			additional_properties: Some(AdditionalProperties::Schema(items)),
			..Default::default()
		})),
		dependencies
	}
}

impl_openapi_type!(
	BTreeMap<K: OpenapiType, T: OpenapiType>,
	IndexMap<K: OpenapiType, T: OpenapiType>,
	HashMap<K: OpenapiType, T: OpenapiType, S: BuildHasher>
	=> map_schema::<K, T>()
);
