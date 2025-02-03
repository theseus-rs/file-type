use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2609693029: FileFormat = FileFormat {
    id: 2_609_693_029,
    source_type: SourceType::Iana,
    name: "automationml-amlx+zip",
    extensions: &[],
    media_types: &["application/automationml-amlx+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
