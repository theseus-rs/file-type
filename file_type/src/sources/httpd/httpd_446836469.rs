use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_446836469: FileType = FileType {
    file_format: &FileFormat {
        id: 446_836_469,
        source_type: SourceType::Httpd,
        name: "wap wbxml",
        extensions: &["wbxml"],
        media_types: &["application/vnd.wap.wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
