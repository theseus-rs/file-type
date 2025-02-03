use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3368099385: FileFormat = FileFormat {
    id: 3_368_099_385,
    source_type: SourceType::Iana,
    name: "mathml+xml",
    extensions: &[],
    media_types: &["application/mathml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
