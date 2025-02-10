use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2819658116: FileType = FileType {
    file_format: &FileFormat {
        id: 2_819_658_116,
        source_type: SourceType::Httpd,
        name: "amazon ebook",
        extensions: &["azw"],
        media_types: &["application/vnd.amazon.ebook"],
        signatures: &[],
        related_formats: &[],
    },
};
