use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_709074434: FileType = FileType {
    file_format: &FileFormat {
        id: 709_074_434,
        source_type: SourceType::Httpd,
        name: "heic",
        extensions: &["heic"],
        media_types: &["image/heic"],
        signatures: &[],
        related_formats: &[],
    },
};
