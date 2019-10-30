#![cfg(feature = "blanket_impl")]
#![feature(never_type)]

use unwrap_infallible::UnwrapInfallible;

enum MyNeverToken {}

impl From<MyNeverToken> for ! {
    fn from(never: MyNeverToken) -> Self {
        match never {}
    }
}

#[test]
fn with_custom_type() {
    let r: Result<bool, MyNeverToken> = Ok(true);
    assert!(r.unwrap_infallible());
}
