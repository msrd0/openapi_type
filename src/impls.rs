use crate::{OpenapiSchema, OpenapiType, Visitor};
use indexmap::{IndexMap, IndexSet};
use openapiv3::{
	AdditionalProperties, ArrayType, IntegerType, NumberFormat, NumberType, ObjectType, ReferenceOr, SchemaKind,
	StringFormat, StringType, Type, VariantOrUnknownOrEmpty
};
use serde_json::Value;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	ffi::{CStr, CString},
	hash::BuildHasher,
	num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize}
};

impl OpenapiType for () {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		visitor.visit_unit();
	}
}

impl OpenapiType for Value {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		visitor.visit_any();
	}
}

impl OpenapiType for bool {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		visitor.visit_bool();
	}
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! int {
	($($ty:ident($byte:expr, $minimum:expr);)+) => {
		$(
			impl OpenapiType for $ty {
				fn visit_type<V: Visitor>(visitor: &mut V) {
					visitor.visit_int($byte, $minimum);
				}
			}
		)+
	}
}

int! {
	isize(None, None);
	i8(None, Some(1));
	i16(None, Some(2));
	i32(None, Some(4));
	i64(None, Some(8));
	i128(None, Some(16));

	usize(Some(0), None);
	u8(Some(0), Some(1));
	u16(Some(0), Some(2));
	u32(Some(0), Some(4));
	u64(Some(0), Some(8));
	u128(Some(0), Some(16));

	NonZeroUsize(Some(1), None);
	NonZeroU8(Some(1), Some(1));
	NonZeroU16(Some(1), Some(2));
	NonZeroU32(Some(1), Some(4));
	NonZeroU64(Some(1), Some(8));
	NonZeroU128(Some(1), Some(16));
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! number {
	($($ty:ident($byte:expr);)+) => {
		$(
			impl OpenapiType for $ty {
				fn visit_type<V: Visitor>(visitor: &mut V) {
					visitor.visit_number($byte);
				}
			}
		)+
	}
}

number! {
	f32(Some(4));
	f64(Some(8));
}

////////////////////////////////////////////////////////////////////////////////

impl OpenapiType for char {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		visitor.visit_char();
	}
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! string {
	($($ty:ident;)+) => {
		$(
			impl OpenapiType for $ty {
				fn visit_type<V: Visitor>(visitor: &mut V) {
					visitor.visit_string();
				}
			}
		)+
	}
}

string! {
	String;
	str;
	CString;
	CStr;
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "uuid08")]
impl OpenapiType for uuid08::Uuid {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		visitor.visit_uuid();
	}
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! date {
	($($($ty:ident)::+$(<$arg:ident: $bound:path>)?;)+) => {
		$(
			impl$(<$arg: $bound>)? OpenapiType for $($ty)::+$(<$arg>)? {
				fn visit_type<V: Visitor>(visitor: &mut V) {
					visitor.visit_date();
				}
			}
		)+
	}
}

#[cfg(feature = "time03")]
date! {
	time03::Date;
}

#[cfg(feature = "chrono04")]
date! {
	chrono04::Date<T: chrono04::TimeZone>;
	chrono04::NaiveDate;
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! datetime {
	($($($ty:ident)::+$(<$arg:ident: $bound:path>)?;)+) => {
		$(
			impl$(<$arg: $bound>)? OpenapiType for $($ty)::+$(<$arg>)? {
				fn visit_type<V: Visitor>(visitor: &mut V) {
					visitor.visit_datetime();
				}
			}
		)+
	}
}

#[cfg(feature = "time03")]
date! {
	time03::OffsetDateTime;
	time03::PrimitiveDateTime;
}

#[cfg(feature = "chrono04")]
date! {
	chrono04::DateTime<T: chrono04::TimeZone>;
	chrono04::NaiveDateTime;
}

////////////////////////////////////////////////////////////////////////////////

impl<T: OpenapiType> OpenapiType for Option<T> {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		let v = visitor.visit_option();
		T::visit_type(v);
	}
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_openapi_type {
	($($($ty:ident)::+ $(<$($generic:ident : $bound:path),+>)?),* => $schema:expr) => {
		$(
			impl $(<$($generic : $bound),+>)? OpenapiType for $($ty)::+ $(<$($generic),+>)? {
				fn schema() -> OpenapiSchema {
					$schema
				}
			}
		)*
	};
}

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
			items: Some(items),
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
	HashMap<K: OpenapiType, T: OpenapiType, S: BuildHasher>,
	IndexMap<K: OpenapiType, T: OpenapiType>
	=> map_schema::<K, T>()
);

#[cfg(feature = "linked-hash-map")]
impl_openapi_type!(
	linked_hash_map::LinkedHashMap<K: OpenapiType, T: OpenapiType, S: BuildHasher>
	=> map_schema::<K, T>()
);
