# Encrust
[![Build Status](https://travis-ci.org/ianlet/encrust.svg?branch=master)](https://travis-ci.org/ianlet/encrust)

Encrust is a testing framework for Rust (nightly) to support Behaviour-Driven Development (BDD).
It is focused on making acceptance tests easy to implement and to maintain. Here's an example of
what a complete test suite would look like:

```rust
#![feature(plugin)]
#![plugin(encrust_codegen)]

extern crate encrust;
extern crate my_super_crate;

use my_super_crate::Belly;

#[context]
#[derive(Default)]
struct FeatureContext {
    pub belly: Belly,
}

#[given("my belly is empty")]
fn my_belly_is_empty(context: &mut FeatureContext) {
    context.belly.empty();
}

#[when("I eat :cuke_count cukes")]
fn i_eat_cukes(cuke_count: i32, context: &mut FeatureContext) {
    context.belly.eat(cuke_count);
}

#[then("my belly should be full")]
fn my_belly_should_be_full(context: &mut FeatureContext) {
    assert!(context.belly.is_full());
}
```

## Documentation
Coming soon...

## License

Encrust is licensed under either of the following, at your option:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
