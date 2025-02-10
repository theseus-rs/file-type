use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1102824002: FileType = FileType {
    file_format: &FileFormat {
        id: 1_102_824_002,
        source_type: SourceType::Iana,
        name: "vnd.vivo",
        extensions: &[],
        media_types: &["video/vnd.vivo"],
        signatures: &[],
        related_formats: &[],
    },
};
