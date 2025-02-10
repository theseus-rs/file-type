use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1368694713: FileType = FileType {
    file_format: &FileFormat {
        id: 1_368_694_713,
        source_type: SourceType::Httpd,
        name: "msword",
        extensions: &["doc", "dot"],
        media_types: &["application/msword"],
        signatures: &[],
        related_formats: &[],
    },
};
