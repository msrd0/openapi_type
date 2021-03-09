use openapi_type::OpenapiType;

#[derive(OpenapiType)]
enum Foo {
	Pair(i64, i64)
}

fn main() {}
