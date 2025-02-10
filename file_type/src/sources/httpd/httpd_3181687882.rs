use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3181687882: FileType = FileType {
    file_format: &FileFormat {
        id: 3_181_687_882,
        source_type: SourceType::Httpd,
        name: "groove tool message",
        extensions: &["gtm"],
        media_types: &["application/vnd.groove-tool-message"],
        signatures: &[],
        related_formats: &[],
    },
};
