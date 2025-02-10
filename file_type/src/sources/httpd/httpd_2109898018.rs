use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2109898018: FileType = FileType {
    file_format: &FileFormat {
        id: 2_109_898_018,
        source_type: SourceType::Httpd,
        name: "woff2",
        extensions: &["woff2"],
        media_types: &["font/woff2"],
        signatures: &[],
        related_formats: &[],
    },
};
