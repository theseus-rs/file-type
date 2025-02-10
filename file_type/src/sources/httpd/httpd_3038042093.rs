use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3038042093: FileType = FileType {
    file_format: &FileFormat {
        id: 3_038_042_093,
        source_type: SourceType::Httpd,
        name: "portable anymap",
        extensions: &["pnm"],
        media_types: &["image/x-portable-anymap"],
        signatures: &[],
        related_formats: &[],
    },
};
