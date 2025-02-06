use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_643161436: FileFormat = FileFormat {
    id: 643_161_436,
    source_type: SourceType::Iana,
    name: "EmergencyCallData.LegacyESN+json",
    extensions: &[],
    media_types: &["application/EmergencyCallData.LegacyESN+json"],
    signatures: &[],
    related_formats: &[],
};
