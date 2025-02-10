use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_282299330: FileType = FileType {
    file_format: &FileFormat {
        id: 282_299_330,
        source_type: SourceType::Httpd,
        name: "msterminal",
        extensions: &["trm"],
        media_types: &["application/x-msterminal"],
        signatures: &[],
        related_formats: &[],
    },
};
