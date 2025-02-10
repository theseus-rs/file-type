use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3000733719: FileType = FileType {
    file_format: &FileFormat {
        id: 3_000_733_719,
        source_type: SourceType::Httpd,
        name: "rfc822",
        extensions: &["eml", "mime"],
        media_types: &["message/rfc822"],
        signatures: &[],
        related_formats: &[],
    },
};
