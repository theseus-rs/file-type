use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_29465275: FileType = FileType {
    file_format: &FileFormat {
        id: 29_465_275,
        source_type: SourceType::Httpd,
        name: "trueapp",
        extensions: &["tra"],
        media_types: &["application/vnd.trueapp"],
        signatures: &[],
        related_formats: &[],
    },
};
