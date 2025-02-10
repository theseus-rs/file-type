use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3102304575: FileType = FileType {
    file_format: &FileFormat {
        id: 3_102_304_575,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.ProviderInfo+xml",
        extensions: &[],
        media_types: &["application/EmergencyCallData.ProviderInfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
