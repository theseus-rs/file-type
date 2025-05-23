use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
