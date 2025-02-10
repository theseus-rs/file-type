use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4029535853: FileType = FileType {
    file_format: &FileFormat {
        id: 4_029_535_853,
        source_type: SourceType::Httpd,
        name: "webp",
        extensions: &["webp"],
        media_types: &["image/webp"],
        signatures: &[],
        related_formats: &[],
    },
};
