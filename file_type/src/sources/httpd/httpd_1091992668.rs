use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1091992668: FileType = FileType {
    file_format: &FileFormat {
        id: 1_091_992_668,
        source_type: SourceType::Httpd,
        name: "nuera ecelp9600",
        extensions: &["ecelp9600"],
        media_types: &["audio/vnd.nuera.ecelp9600"],
        signatures: &[],
        related_formats: &[],
    },
};
