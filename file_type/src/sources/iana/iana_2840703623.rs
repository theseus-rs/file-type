use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2840703623: FileType = FileType {
    file_format: &FileFormat {
        id: 2_840_703_623,
        source_type: SourceType::Iana,
        name: "EmergencyCallData.cap+xml",
        extensions: &[],
        media_types: &["application/EmergencyCallData.cap+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
