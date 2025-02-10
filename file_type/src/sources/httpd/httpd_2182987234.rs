use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2182987234: FileType = FileType {
    file_format: &FileFormat {
        id: 2_182_987_234,
        source_type: SourceType::Httpd,
        name: "json",
        extensions: &["json"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
