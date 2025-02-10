use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3549948928: FileType = FileType {
    file_format: &FileFormat {
        id: 3_549_948_928,
        source_type: SourceType::Httpd,
        name: "pls xml",
        extensions: &["pls"],
        media_types: &["application/pls+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
