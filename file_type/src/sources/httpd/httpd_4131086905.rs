use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4131086905: FileType = FileType {
    file_format: &FileFormat {
        id: 4_131_086_905,
        source_type: SourceType::Httpd,
        name: "geometry explorer",
        extensions: &["gex", "gre"],
        media_types: &["application/vnd.geometry-explorer"],
        signatures: &[],
        related_formats: &[],
    },
};
