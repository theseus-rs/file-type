use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3549948928: FileFormat = FileFormat {
    id: 3_549_948_928,
    source_type: SourceType::Iana,
    name: "pls+xml",
    extensions: &[],
    media_types: &["application/pls+xml"],
    signatures: &[],
    related_formats: &[],
};
