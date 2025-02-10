use crate::FileType;

pub(crate) mod custom_1;
pub(crate) mod custom_2;
pub(crate) mod custom_3;
pub(crate) mod custom_4;
pub(crate) mod custom_5;

pub(crate) use custom_1::CUSTOM_1;
pub(crate) use custom_2::CUSTOM_2;
pub(crate) use custom_3::CUSTOM_3;
pub(crate) use custom_4::CUSTOM_4;
pub(crate) use custom_5::CUSTOM_5;

#[doc(hidden)]
pub const FILE_TYPES: &[&FileType] = &[&CUSTOM_1, &CUSTOM_2, &CUSTOM_3, &CUSTOM_4, &CUSTOM_5];
