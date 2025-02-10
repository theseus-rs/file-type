use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_643161436: FileType = FileType {
    file_format: &FileFormat {
        id: 643_161_436,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.LegacyESN+json",
        extensions: &[],
        media_types: &["application/EmergencyCallData.LegacyESN+json"],
        signatures: &[],
        related_formats: &[],
    },
};
