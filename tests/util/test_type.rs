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
	($ty:ty = $json:tt, {$($dep_name:literal: $dep_json:tt),*}) => {
		test_type!($ty = $json);
		paste::paste! {
			#[test]
			fn [< $ty:lower _dependencies >]() {
				let mut schema = <$ty as OpenapiType>::schema();
				$({
					let dep_schema = schema.dependencies.remove($dep_name).expect(concat!("Schema is missing the following dependency: ", $dep_name));
					let dep_schema = openapi_type::OpenapiSchema::into_schema(dep_schema);
					let dep_json = serde_json::to_value(&dep_schema).unwrap();
					let expected = serde_json::json!($dep_json);
					pretty_assertions::assert_eq!(dep_json, expected)
				})*
			}
		}
	};
}
