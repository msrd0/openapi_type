# openapi_type [![Rust 1.43+](https://img.shields.io/badge/rustc-1.43+-orange.svg)](https://blog.rust-lang.org/2020/04/23/Rust-1.43.0.html) [![License Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0) [![GitHub](https://img.shields.io/badge/Code-On%20Github-blue?logo=GitHub)](https://github.com/msrd0/openapi_type)

This crate gives static type information for primitives and commonly used types from the standard
library and a few other commonly used libraries like `chrono` and `uuid`. Also, it provides a
derive macro for structs and enums to gain access to their static type information at runtime.

The core of this crate is the [`OpenapiType`] trait. It has one static function,
[`schema`](OpenapiType::schema), which returns an [`OpenapiSchema`]. This assembles the static
type information in a way that is convenient to use for a generated OpenAPI specification, but
can also be utilized in other use cases as well.

## Custom Types
To gain access to the static type information of your custom types at runtime, the easiest way
is to use the derive macro:

```rust
#[derive(OpenapiType)]
struct FooBar {
	foo: String,
	bar: u64
}
```

## OpenAPI specification
Using above type, running `FooBar::schema().into_schema()` yields

```yaml
type: object
title: FooBar
properties:
  foo:
    type: string
  bar:
    type: integer
    format: int64
    minimum: 0
required:
  - foo
  - bar
```

Note, however, that this is not sufficient for more complex types. If one of your structs fields
is a type that has a name (that is, `Type::schema().name` is not `None`), above schema will contain
a reference to that schema. Therefore, always remember to put the
[`dependencies`](OpenapiSchema::dependencies) into the specification alongside the type you are
interested in.

## Versioning

Like all rust crates, this crate will follow semantic versioning guidelines. However, changing
the MSRV (minimum supported rust version) is not considered a breaking change.

## License

Copyright (C) 2021 Dominic Meiser and [contributors](https://github.com/msrd0/openapi_type/graphs/contributors).

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

	https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```