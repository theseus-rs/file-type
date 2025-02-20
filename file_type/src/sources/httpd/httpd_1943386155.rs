use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1943386155: FileType = FileType {
    file_format: &FileFormat {
        id: 1_943_386_155,
        source_type: SourceType::Httpd,
        name: "tmobile livetv",
        extensions: &["tmo"],
        media_types: &["application/vnd.tmobile-livetv"],
        signatures: &[],
        related_formats: &[],
    },
};
