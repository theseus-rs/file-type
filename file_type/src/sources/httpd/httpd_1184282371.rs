use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1184282371: FileType = FileType {
    file_format: &FileFormat {
        id: 1_184_282_371,
        source_type: SourceType::Httpd,
        name: "ccxml xml",
        extensions: &["ccxml"],
        media_types: &["application/ccxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
