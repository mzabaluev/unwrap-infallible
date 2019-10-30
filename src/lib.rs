//! Conversion method for infallible results
//!
//! This crate provides a convenience trait `UnwrapInfallible`,
//! adding method `unwrap_infallible` to `Result` types where an `Err` variant
//! is statically known to never occur.
//!
//! # Example
//!
//! ```
//! use unwrap_infallible::UnwrapInfallible;
//! use std::convert::Infallible;
//!
//! fn always_sunny() -> Result<String, Infallible> {
//!     Ok("it's always sunny!".into())
//! }
//!
//! fn print_weather() {
//!     println!("{}", always_sunny().unwrap_infallible());
//! }
//! ```

#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(missing_docs)]

#![cfg_attr(feature = "never_type", feature(never_type))]

use core::hint::unreachable_unchecked;

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
impl<T, E> UnwrapInfallible for Result<T, E>
where
    E: From<!>,
{
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|_| {
            unsafe { unreachable_unchecked() }
        })
    }
}

#[cfg(all(feature = "never_type", not(feature = "blanket_impl")))]
impl<T> UnwrapInfallible for Result<T, !> {
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|_| {
            unsafe { unreachable_unchecked() }
        })
    }
}

#[cfg(not(feature = "blanket_impl"))]
impl<T> UnwrapInfallible for Result<T, Infallible> {
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|_: Infallible| {
            unsafe { unreachable_unchecked() }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::UnwrapInfallible;
    use core::convert::TryFrom;
    use core::hint::unreachable_unchecked;

    #[test]
    fn with_infallible() {
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

    enum MyNeverToken {}

    #[cfg(feature = "never_type")]
    impl From<!> for MyNeverToken {
        fn from(_: !) -> Self {
            unsafe { unreachable_unchecked() }
        }
    }

    #[cfg(not(feature = "blanket_impl"))]
    impl<T> UnwrapInfallible for Result<T, MyNeverToken> {
        type Ok = T;
        fn unwrap_infallible(self) -> T {
            self.unwrap_or_else(|_| {
                unsafe { unreachable_unchecked() }
            })
        }
    }

    #[test]
    fn with_custom_type() {
        let r: Result<bool, MyNeverToken> = Ok(true);
        assert!(r.unwrap_infallible());
    }
}
