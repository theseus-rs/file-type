use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_472064312: FileFormat = FileFormat {
    id: 472_064_312,
    source_type: SourceType::Iana,
    name: "cccex",
    extensions: &[],
    media_types: &["application/cccex"],
    signatures: &[],
    related_formats: &[],
};
