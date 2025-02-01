use crate::format::FileFormat;

mod custom_1;
mod custom_2;
mod custom_3;
mod custom_4;
mod custom_5;

pub(crate) const FILE_FORMATS: &[&FileFormat] = &[
    &custom_1::CUSTOM_1,
    &custom_2::CUSTOM_2,
    &custom_3::CUSTOM_3,
    &custom_4::CUSTOM_4,
    &custom_5::CUSTOM_5,
];
