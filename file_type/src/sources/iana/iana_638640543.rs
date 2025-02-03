use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_638640543: FileFormat = FileFormat {
    id: 638_640_543,
    source_type: SourceType::Iana,
    name: "csta+xml",
    extensions: &[],
    media_types: &["application/csta+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
