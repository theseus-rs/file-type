use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3923222572: FileType = FileType {
    file_format: &FileFormat {
        id: 3_923_222_572,
        source_type: SourceType::Httpd,
        name: "groove identity message",
        extensions: &["gim"],
        media_types: &["application/vnd.groove-identity-message"],
        signatures: &[],
        related_formats: &[],
    },
};
