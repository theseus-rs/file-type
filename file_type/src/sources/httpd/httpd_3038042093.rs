use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
