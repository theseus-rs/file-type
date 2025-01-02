#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

use file_type::FileType;

fuzz_target!(|data: &[u8]| {
    let _ = FileType::from_bytes(data);
});
