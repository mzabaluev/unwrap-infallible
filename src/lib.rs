#![cfg_attr(feature = "never_type", feature(never_type))]

use core::hint::unreachable_unchecked;

#[cfg(not(feature = "never_type"))]
use core::convert::Infallible;

pub trait UnwrapInfallible {
    type Ok;
    fn unwrap_infallible(self) -> Self::Ok;
}

#[cfg(feature = "never_type")]
impl<T, E: From<!>> UnwrapInfallible for Result<T, E> {
    type Ok = T;
    fn unwrap_infallible(self) -> T {
        self.unwrap_or_else(|_| {
            unsafe { unreachable_unchecked() }
        })
    }
}

#[cfg(not(feature = "never_type"))]
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

    #[cfg(not(feature = "never_type"))]
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
