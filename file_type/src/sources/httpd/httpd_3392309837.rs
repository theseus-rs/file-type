use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3392309837: FileType = FileType {
    file_format: &FileFormat {
        id: 3_392_309_837,
        source_type: SourceType::Httpd,
        name: "rss xml",
        extensions: &["rss"],
        media_types: &["application/rss+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
