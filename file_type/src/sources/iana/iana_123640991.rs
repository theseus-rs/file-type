use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_123640991: FileFormat = FileFormat {
    id: 123_640_991,
    source_type: SourceType::Iana,
    name: "whoispp-query",
    extensions: &[],
    media_types: &["application/whoispp-query"],
    internal_signatures: &[],
    related_formats: &[],
};
