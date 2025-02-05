use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2738674935: FileFormat = FileFormat {
    id: 2_738_674_935,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.VEDS+xml",
    extensions: &[],
    media_types: &["application/EmergencyCallData.VEDS+xml"],
    signatures: &[],
    related_formats: &[],
};
