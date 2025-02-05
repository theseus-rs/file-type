use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1537757002: FileFormat = FileFormat {
    id: 1_537_757_002,
    source_type: SourceType::Iana,
    name: "EDIFACT",
    extensions: &[],
    media_types: &["application/EDIFACT"],
    signatures: &[],
    related_formats: &[],
};
