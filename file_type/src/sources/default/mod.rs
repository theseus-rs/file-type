use crate::format::FileFormat;

pub(crate) mod default_1;
pub(crate) mod default_2;

pub(crate) use default_1::DEFAULT_1;
pub(crate) use default_2::DEFAULT_2;

pub(crate) const FILE_FORMATS: &[&FileFormat] = &[&DEFAULT_1, &DEFAULT_2];
