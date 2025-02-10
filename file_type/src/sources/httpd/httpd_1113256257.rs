use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1113256257: FileType = FileType {
    file_format: &FileFormat {
        id: 1_113_256_257,
        source_type: SourceType::Httpd,
        name: "sgml",
        extensions: &["sgml", "sgm"],
        media_types: &["text/sgml"],
        signatures: &[],
        related_formats: &[],
    },
};
