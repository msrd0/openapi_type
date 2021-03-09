use openapi_type::OpenapiType;

#[derive(OpenapiType)]
struct Foo<T> {
	bar: T
}

struct Bar;

fn main() {
	<Foo<Bar>>::schema();
}
