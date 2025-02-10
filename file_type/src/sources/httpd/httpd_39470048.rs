use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_39470048: FileType = FileType {
    file_format: &FileFormat {
        id: 39_470_048,
        source_type: SourceType::Httpd,
        name: "pdf",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
