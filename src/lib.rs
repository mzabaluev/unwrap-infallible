//! Conversion method for infallible results
//!
//! This crate provides a convenience trait `UnwrapInfallible`,
//! adding method `unwrap_infallible` to `Result` types where an `Err` variant
//! is statically known to never occur.
//!
//! # Example
//!
//! ```
//! # #![cfg_attr(feature = "never_type", feature(never_type))]
//! #
//! use unwrap_infallible::UnwrapInfallible;
//! # #[cfg(not(feature = "blanket_impl"))]
//! use std::convert::Infallible;
//! # #[cfg(feature = "blanket_impl")]
//! # type Infallible = !;
//!
//! fn always_sunny() -> Result<String, Infallible> {
//!     Ok("it's always sunny!".into())
//! }
//!
//! fn main() {
//!     println!("{}", always_sunny().unwrap_infallible());
//! }
//! ```

#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![no_std]
#![cfg_attr(feature = "never_type", feature(never_type))]

#[cfg(not(feature = "blanket_impl"))]
use core::convert::Infallible;

/// Unwrapping an infallible result into its success value.
pub trait UnwrapInfallible {
    /// Type of the `Ok` variant of the result.
    type Ok;

    /// Unwraps a result, returning the content of an `Ok`.
    ///
    /// Unlike `Result::unwrap`, this method is known to never panic
    /// on the result types it is implemented for. Therefore, it can be used
    /// instead of `unwrap` as a maintainability safeguard that will fail
    /// to compile if the error type of the `Result` is later changed
    /// to an error that can actually occur.
    fn unwrap_infallible(self) -> Self::Ok;
}

#[cfg(feature = "blanket_impl")]
impl<T, E: Into<!>> UnwrapInfallible for Result<T, E> {
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => e.into(),
        }
    }
}

#[cfg(all(feature = "never_type", not(feature = "blanket_impl")))]
impl<T> UnwrapInfallible for Result<T, !> {
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|never| never)
    }
}

#[cfg(not(feature = "blanket_impl"))]
impl<T> UnwrapInfallible for Result<T, Infallible> {
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|never| match never {})
    }
}

#[cfg(test)]
mod tests {
    use super::UnwrapInfallible;

    // Hmm, Infallible is not Into<!> yet
    #[cfg(not(feature = "blanket_impl"))]
    #[test]
    fn with_infallible() {
        use core::convert::TryFrom;

        let a = 42u8;
        let a = u64::try_from(a).unwrap_infallible();
        assert_eq!(a, 42u64);
    }

    #[cfg(feature = "never_type")]
    #[test]
    fn with_never_type() {
        let r: Result<bool, !> = Ok(true);
        assert!(r.unwrap_infallible());
    }
}
