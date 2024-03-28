#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MyType(u64);

impl MyType {
    pub fn new(v: u64) -> Self {
        Self(v)
    }
}

uniffi::custom_newtype!(MyType, u64);

#[derive(Debug, Copy, Clone, uniffi::Enum)]
pub enum MyEnum {
    Foo,
    Bar,
}

uniffi::setup_scaffolding!();
