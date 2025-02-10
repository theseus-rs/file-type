use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3921585630: FileType = FileType {
    file_format: &FileFormat {
        id: 3_921_585_630,
        source_type: SourceType::Httpd,
        name: "mpegurl",
        extensions: &["mxu", "m4u"],
        media_types: &["video/vnd.mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
