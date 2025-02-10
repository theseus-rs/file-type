use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2645702580: FileType = FileType {
    file_format: &FileFormat {
        id: 2_645_702_580,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.DeviceInfo+xml",
        extensions: &[],
        media_types: &["application/EmergencyCallData.DeviceInfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
