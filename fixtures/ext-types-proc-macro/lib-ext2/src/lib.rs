use lib_ext::{MyEnum, MyType};
#[derive(uniffi::Record)]
pub struct MyRecord {
    pub name: String,
    pub value: MyType,
    pub my_enum: MyEnum,
}

impl MyRecord {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: MyType::new(24),
            my_enum: MyEnum::Bar,
        }
    }
}
uniffi::setup_scaffolding!();

uniffi::ffi_converter_forward!(MyType, lib_ext::UniFfiTag, UniFfiTag);
