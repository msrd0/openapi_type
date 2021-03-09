use openapi_type::OpenapiType;

#[derive(OpenapiType)]
struct Foo {
	bar: Bar
}

struct Bar;

fn main() {
	Foo::schema();
}
