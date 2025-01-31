use crate::format::FileFormat;

mod default_1;
mod default_2;

pub(crate) const FILE_FORMATS: &[&FileFormat] = &[&default_1::DEFAULT_1, &default_2::DEFAULT_2];
