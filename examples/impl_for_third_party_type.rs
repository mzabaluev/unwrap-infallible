// This example demonstrates how UnwrapInfallible can be implemented for
// types other than Result. The implementation has to be in the same crate
// as the type for which it is defined.

use unwrap_infallible::UnwrapInfallible;

pub enum Outcome<T, U> {
    Normal(T),
    Unexpected(U),
}

pub enum MyNeverType {}

impl<T> UnwrapInfallible for Outcome<T, MyNeverType> {
    type Ok = T;

    fn unwrap_infallible(self) -> T {
        match self {
            Outcome::Normal(v) => v,
            Outcome::Unexpected(u) => match u {},
        }
    }
}

fn handle_outcome(o: Outcome<i32, MyNeverType>) {
    let v = o.unwrap_infallible();
    println!("{v}");
}

fn main() {
    handle_outcome(Outcome::Normal(42));
}
