use crate::FileType;

pub(crate) mod default_1;
pub(crate) mod default_2;

pub(crate) use default_1::DEFAULT_1;
pub(crate) use default_2::DEFAULT_2;

#[doc(hidden)]
pub const FILE_TYPES: &[&FileType] = &[&DEFAULT_1, &DEFAULT_2];
