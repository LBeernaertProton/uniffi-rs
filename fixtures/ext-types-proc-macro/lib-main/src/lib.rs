use lib_ext2::MyRecord;

#[uniffi::export]
pub fn get_record() -> MyRecord {
    MyRecord::new("MyRecord".to_string())
}

uniffi::setup_scaffolding!();
