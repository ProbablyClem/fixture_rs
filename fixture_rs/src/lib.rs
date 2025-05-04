pub use fixture_derive::Fixture;

pub trait Fixture {
    fn fixture() -> Self;
}

impl Fixture for String {
    fn fixture() -> Self {
        "string".to_string()
    }
}

impl Fixture for u32 {
    fn fixture() -> Self {
        1
    }
}

impl<T: Fixture> Fixture for Option<T> {
    fn fixture() -> Self {
        Some(T::fixture())
    }
}

impl<T: Fixture> Fixture for Vec<T> {
    fn fixture() -> Self {
        vec![T::fixture()]
    }
}

impl<T: Fixture> Fixture for Box<T> {
    fn fixture() -> Self {
        Box::new(T::fixture())
    }
}
