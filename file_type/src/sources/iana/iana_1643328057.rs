use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1643328057: FileFormat = FileFormat {
    id: 1_643_328_057,
    source_type: SourceType::Iana,
    name: "stix+json",
    extensions: &[],
    media_types: &["application/stix+json"],
    signatures: &[],
    related_formats: &[],
};
