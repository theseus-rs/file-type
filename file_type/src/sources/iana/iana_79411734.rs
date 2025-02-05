use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_79411734: FileFormat = FileFormat {
    id: 79_411_734,
    source_type: SourceType::Iana,
    name: "svg+xml",
    extensions: &[],
    media_types: &["image/svg+xml"],
    signatures: &[],
    related_formats: &[],
};
