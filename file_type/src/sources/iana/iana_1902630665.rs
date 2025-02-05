use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1902630665: FileFormat = FileFormat {
    id: 1_902_630_665,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.Comment+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.Comment+xml"],
    signatures: &[],
    related_formats: &[],
};
