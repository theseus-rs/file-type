use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2132147605: FileType = FileType {
    file_format: &FileFormat {
        id: 2_132_147_605,
        source_type: SourceType::Httpd,
        name: "webm",
        extensions: &["webm"],
        media_types: &["video/webm"],
        signatures: &[],
        related_formats: &[],
    },
};
