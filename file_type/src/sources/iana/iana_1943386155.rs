use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1943386155: FileType = FileType {
    file_format: &FileFormat {
        id: 1_943_386_155,
        source_type: SourceType::Iana,
        name: "vnd.tmobile-livetv",
        extensions: &[],
        media_types: &["application/vnd.tmobile-livetv"],
        signatures: &[],
        related_formats: &[],
    },
};
