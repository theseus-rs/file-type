use crate::FileType;

pub(crate) mod custom_1;
pub(crate) mod custom_2;
pub(crate) mod custom_3;
pub(crate) mod custom_4;
pub(crate) mod custom_5;

#[doc(hidden)]
pub const FILE_TYPES: &[&FileType] = &[
    &custom_1::CUSTOM_1,
    &custom_2::CUSTOM_2,
    &custom_3::CUSTOM_3,
    &custom_4::CUSTOM_4,
    &custom_5::CUSTOM_5,
];
