use openapi_type::OpenapiType;

#[derive(OpenapiType)]
union Foo {
	signed: i64,
	unsigned: u64
}

fn main() {}
