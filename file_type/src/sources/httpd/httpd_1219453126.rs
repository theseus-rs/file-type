use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1219453126: FileType = FileType {
    file_format: &FileFormat {
        id: 1_219_453_126,
        source_type: SourceType::Httpd,
        name: "dart",
        extensions: &["dart"],
        media_types: &["application/vnd.dart"],
        signatures: &[],
        related_formats: &[],
    },
};
