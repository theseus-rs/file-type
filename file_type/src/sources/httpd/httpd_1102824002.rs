use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1102824002: FileType = FileType {
    file_format: &FileFormat {
        id: 1_102_824_002,
        source_type: SourceType::Httpd,
        name: "vivo",
        extensions: &["viv"],
        media_types: &["video/vnd.vivo"],
        signatures: &[],
        related_formats: &[],
    },
};
