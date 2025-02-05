use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_98475931: FileFormat = FileFormat {
    id: 98_475_931,
    source_type: SourceType::Iana,
    name: "manifest+json",
    extensions: &[],
    media_types: &["application/manifest+json"],
    signatures: &[],
    related_formats: &[],
};
