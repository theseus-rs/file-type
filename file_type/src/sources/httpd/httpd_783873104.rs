use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_783873104: FileType = FileType {
    file_format: &FileFormat {
        id: 783_873_104,
        source_type: SourceType::Httpd,
        name: "opml",
        extensions: &["opml"],
        media_types: &["text/x-opml"],
        signatures: &[],
        related_formats: &[],
    },
};
