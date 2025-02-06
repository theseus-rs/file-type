use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1988814640: FileFormat = FileFormat {
    id: 1_988_814_640,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.Control+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.Control+xml"],
    signatures: &[],
    related_formats: &[],
};
