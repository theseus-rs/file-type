use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1432336035: FileFormat = FileFormat {
    id: 1_432_336_035,
    source_type: SourceType::Iana,
    name: "lpf+zip",
    extensions: &[],
    media_types: &["application/lpf+zip"],
    signatures: &[],
    related_formats: &[],
};
