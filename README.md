# Unwrapping Results With Compile-Time Guarantee of Infallibility

The Rust standard type `Result` sometimes occurs parameterized
with an error type that has no possible values, such as
`std::convert::Infallible`. Consequently, calling the `unwrap` method on a
`Result` value of such a type will never panic.
Therein lies a maintainability hazard: if the error parameter type is later
changed to one that can represent actually occurring errors, those uses of
`unwrap` that could previously be relied upon to be infallible, quietly
become liable to panic.

To help prevent this from happening without a compile-time safeguard,
this crate provides an alternative method `unwrap_infallible` that shall only
be available for `Result` values with a known-impossible `Err` variant.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
