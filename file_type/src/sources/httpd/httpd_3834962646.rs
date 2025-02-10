use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3834962646: FileType = FileType {
    file_format: &FileFormat {
        id: 3_834_962_646,
        source_type: SourceType::Httpd,
        name: "ssml xml",
        extensions: &["ssml"],
        media_types: &["application/ssml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
