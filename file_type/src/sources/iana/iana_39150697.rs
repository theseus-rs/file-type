use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_39150697: FileFormat = FileFormat {
    id: 39_150_697,
    source_type: SourceType::Iana,
    name: "application/trust-chain+json",
    extensions: &[],
    media_types: &["application/trust-chain+json"],
    internal_signatures: &[],
    related_formats: &[],
};
