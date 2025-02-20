use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1353407983: FileType = FileType {
    file_format: &FileFormat {
        id: 1_353_407_983,
        source_type: SourceType::Httpd,
        name: "wap wml",
        extensions: &["wml"],
        media_types: &["text/vnd.wap.wml"],
        signatures: &[],
        related_formats: &[],
    },
};
