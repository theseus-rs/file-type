use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1439758621: FileType = FileType {
    file_format: &FileFormat {
        id: 1_439_758_621,
        source_type: SourceType::Httpd,
        name: "m4v",
        extensions: &["m4v"],
        media_types: &["video/x-m4v"],
        signatures: &[],
        related_formats: &[],
    },
};
