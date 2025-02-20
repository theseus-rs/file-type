use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3168124663: FileType = FileType {
    file_format: &FileFormat {
        id: 3_168_124_663,
        source_type: SourceType::Httpd,
        name: "gnumeric",
        extensions: &["gnumeric"],
        media_types: &["application/x-gnumeric"],
        signatures: &[],
        related_formats: &[],
    },
};
