use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3815365048: FileType = FileType {
    file_format: &FileFormat {
        id: 3_815_365_048,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.ServiceInfo+xml",
        extensions: &[],
        media_types: &["application/EmergencyCallData.ServiceInfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
