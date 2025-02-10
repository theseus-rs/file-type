use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4008929422: FileType = FileType {
    file_format: &FileFormat {
        id: 4_008_929_422,
        source_type: SourceType::Httpd,
        name: "csml",
        extensions: &["csml"],
        media_types: &["chemical/x-csml"],
        signatures: &[],
        related_formats: &[],
    },
};
