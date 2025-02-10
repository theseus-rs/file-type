use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_242606803: FileType = FileType {
    file_format: &FileFormat {
        id: 242_606_803,
        source_type: SourceType::Httpd,
        name: "7z compressed",
        extensions: &["7z"],
        media_types: &["application/x-7z-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
