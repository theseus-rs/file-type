use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_151389392: FileFormat = FileFormat {
    id: 151_389_392,
    source_type: SourceType::Iana,
    name: "alto-error+json",
    extensions: &[],
    media_types: &["application/alto-error+json"],
    internal_signatures: &[],
    related_formats: &[],
};
