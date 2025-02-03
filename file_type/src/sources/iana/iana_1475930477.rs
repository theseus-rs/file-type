use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1475930477: FileFormat = FileFormat {
    id: 1_475_930_477,
    source_type: SourceType::Iana,
    name: "alto-networkmap+json",
    extensions: &[],
    media_types: &["application/alto-networkmap+json"],
    internal_signatures: &[],
    related_formats: &[],
};
