use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3164453447: FileFormat = FileFormat {
    id: 3_164_453_447,
    source_type: SourceType::Iana,
    name: "rlmi+xml",
    extensions: &[],
    media_types: &["application/rlmi+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
