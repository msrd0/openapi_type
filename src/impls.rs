use crate::{ObjectVisitor, OpenapiType, Visitor};
use indexmap::{IndexMap, IndexSet};
use serde_json::Value;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
	ffi::{CStr, CString},
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
	($($ty:ident($minimum:expr, $byte:expr);)+) => {
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

#[cfg(feature = "uuid1")]
impl OpenapiType for uuid1::Uuid {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		visitor.visit_uuid();
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(any(feature = "time03", feature = "chrono04"))]
macro_rules! date {
	($($($ty:ident)::+ $(<$arg:ident: $bound:path>)?;)+) => {
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

#[cfg(any(feature = "time03", feature = "chrono04"))]
macro_rules! datetime {
	($($($ty:ident)::+ $(<$arg:ident: $bound:path>)?;)+) => {
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
datetime! {
	time03::OffsetDateTime;
	time03::PrimitiveDateTime;
}

#[cfg(feature = "chrono04")]
datetime! {
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

macro_rules! array {
	($($($ty:ident)::+ $(<$($arg:ident),+>)? ($unique_items:literal, $inner:ident);)+) => {
		$(
			impl$(<$($arg),+>)? OpenapiType for $($ty)::+$(<$($arg),+>)?
			where
				$inner: OpenapiType
			{
				fn visit_type<V: Visitor>(visitor: &mut V) {
					let v = visitor.visit_array(None, $unique_items);
					<$inner as OpenapiType>::visit_type(v);
				}
			}
		)+
	}
}

type Array<T> = [T];

array! {
	Array<T>(false, T);
	LinkedList<T>(false, T);
	Vec<T>(false, T);
	VecDeque<T>(false, T);

	BTreeSet<T>(true, T);
	HashSet<T, S>(true, T);
	IndexSet<T>(true, T);
}

impl<T: OpenapiType, const N: usize> OpenapiType for [T; N] {
	fn visit_type<V: Visitor>(visitor: &mut V) {
		let v = visitor.visit_array(Some(N), false);
		T::visit_type(v);
	}
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! map {
	($($($ty:ident)::+ $(<$($arg:ident$(: $bound:path)?),+>)? ($inner:ident);)+) => {
		$(
			impl$(<$($arg$(: $bound)?),+>)? OpenapiType for $($ty)::+$(<$($arg),+>)?
			where
				$inner: OpenapiType
			{
				fn visit_type<Vi: Visitor>(visitor: &mut Vi) {
					let obj = visitor.visit_object();
					let v = obj.visit_additional();
					<$inner as OpenapiType>::visit_type(v);
				}
			}
		)+
	}
}

map! {
	BTreeMap<K, V>(V);
	HashMap<K, V, S>(V);
	IndexMap<K, V, S>(V);
}

#[cfg(feature = "linked-hash-map05")]
map! {
	linked_hash_map05::LinkedHashMap<K, V, S>(V);
}
