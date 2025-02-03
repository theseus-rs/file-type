use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1774385812: FileFormat = FileFormat {
    id: 1_774_385_812,
    source_type: SourceType::Iana,
    name: "geo+json",
    extensions: &[],
    media_types: &["application/geo+json"],
    internal_signatures: &[],
    related_formats: &[],
};
