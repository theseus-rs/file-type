use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3279796176: FileType = FileType {
    file_format: &FileFormat {
        id: 3_279_796_176,
        source_type: SourceType::Httpd,
        name: "oxps",
        extensions: &["oxps"],
        media_types: &["application/oxps"],
        signatures: &[],
        related_formats: &[],
    },
};
