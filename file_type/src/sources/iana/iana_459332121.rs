use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_459332121: FileFormat = FileFormat {
    id: 459_332_121,
    source_type: SourceType::Iana,
    name: "problem+xml",
    extensions: &[],
    media_types: &["application/problem+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
