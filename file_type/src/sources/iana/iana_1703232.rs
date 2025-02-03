use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1703232: FileFormat = FileFormat {
    id: 1_703_232,
    source_type: SourceType::Iana,
    name: "vnd.geo+json (OBSOLETED by [RFC7946] in favor of application/geo+json)",
    extensions: &[],
    media_types: &["application/vnd.geo+json"],
    internal_signatures: &[],
    related_formats: &[],
};
