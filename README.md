<br/>
<div>
	<a href="https://github.com/msrd0/openapi_type/actions/workflows/rust.yml">
		<img alt="build status" src="https://github.com/msrd0/openapi_type/actions/workflows/rust.yml/badge.svg"/>
	</a>
	<a href="https://msrd0.github.io/openapi_type/tarpaulin-report.html">
		<img alt="coverage report" src="https://msrd0.github.io/openapi_type/coverage.svg"/>
	</a>
	<a href="https://msrd0.github.io/openapi_type/doc/openapi_type/index.html">
		<img alt="rustdoc" src="https://img.shields.io/badge/docs-main-blue.svg"/>
	</a>
    <a href="https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html">
        <img alt="Rust 1.56+" src="https://img.shields.io/badge/rustc-1.56+-orange.svg"/>
    </a>
	<a href="https://www.apache.org/licenses/LICENSE-2.0">
		<img alt="License Apache-2.0" src="https://img.shields.io/badge/license-Apache--2.0-blue.svg"/>
	</a>
</div>
<br/>

This repository contains the following crates:

 - **openapi_type**
   [![openapi_type on crates.io](https://img.shields.io/crates/v/openapi_type.svg)](https://crates.io/crates/openapi_type)
   [![openapi_type on docs.rs](https://docs.rs/openapi_type/badge.svg)](https://docs.rs/openapi_type)
 - **openapi_type_derive**
   [![openapi_type_derive on crates.io](https://img.shields.io/crates/v/openapi_type_derive.svg)](https://crates.io/crates/openapi_type_derive)
   [![openapi_type_derive on docs.rs](https://docs.rs/openapi_type_derive/badge.svg)](https://docs.rs/openapi_type_derive)

# openapi_type

This crate gives static type information for primitives and commonly used types from the standard library and other commonly used libraries `chrono`, `indexmap`, `linked-hash-map`, `time` and `uuid` when the according feature is enabled. Also, it provides a derive macro for structs and enums to gain access to their static type information at runtime.

The core of this crate is the [`OpenapiType`][__link0] trait. It has one static function, [`schema`][__link1], which returns an [`OpenapiSchema`][__link2]. This assembles the static type information in a way that is convenient to use for a generated OpenAPI specification, but can also be utilized in other use cases as well.


## Custom Types

To gain access to the static type information of your custom types at runtime, the easiest way is to use the derive macro:


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

Note, however, that this is not sufficient for more complex types. If one of your structs fields is a type that has a name (that is, `Type::schema().name` is not `None`), above schema will contain a reference to that schema. Therefore, always remember to put the [`dependencies`][__link3] into the specification alongside the type you are interested in.



## Versioning

Like all rust crates, this crate will follow semantic versioning guidelines. However, changing
the MSRV (minimum supported rust version) is not considered a breaking change.

## License

Copyright (C) 2021-2022 Dominic Meiser and [contributors].

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

 [contributors]: https://github.com/msrd0/openapi_type/graphs/contributors
 [__link0]: https://docs.rs/openapi_type/0.3.0/openapi_type/?search=openapi_type::OpenapiType
 [__link1]: https://docs.rs/openapi_type/0.3.0/openapi_type/?search=openapi_type::OpenapiType::schema
 [__link2]: https://docs.rs/openapi_type/0.3.0/openapi_type/?search=openapi_type::OpenapiSchema
 [__link3]: https://docs.rs/openapi_type/0.3.0/openapi_type/?search=openapi_type::OpenapiSchema::dependencies
