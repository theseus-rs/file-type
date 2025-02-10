use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2699999072: FileType = FileType {
    file_format: &FileFormat {
        id: 2_699_999_072,
        source_type: SourceType::Httpd,
        name: "chat",
        extensions: &["chat"],
        media_types: &["application/x-chat"],
        signatures: &[],
        related_formats: &[],
    },
};
