use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3010079007: FileFormat = FileFormat {
    id: 3_010_079_007,
    source_type: SourceType::Iana,
    name: "eat-ucs+json",
    extensions: &[],
    media_types: &["application/eat-ucs+json"],
    internal_signatures: &[],
    related_formats: &[],
};
