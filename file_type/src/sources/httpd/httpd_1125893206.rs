use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1125893206: FileType = FileType {
    file_format: &FileFormat {
        id: 1_125_893_206,
        source_type: SourceType::Httpd,
        name: "ms xbap",
        extensions: &["xbap"],
        media_types: &["application/x-ms-xbap"],
        signatures: &[],
        related_formats: &[],
    },
};
