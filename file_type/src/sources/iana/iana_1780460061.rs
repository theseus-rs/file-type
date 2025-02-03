use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1780460061: FileFormat = FileFormat {
    id: 1_780_460_061,
    source_type: SourceType::Iana,
    name: "mmt-usd+xml",
    extensions: &[],
    media_types: &["application/mmt-usd+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
