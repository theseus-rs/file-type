use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_123640991: FileType = FileType {
    file_format: &FileFormat {
        id: 123_640_991,
        source_type: SourceType::Iana,
        name: "whoispp-query",
        extensions: &[],
        media_types: &["application/whoispp-query"],
        signatures: &[],
        related_formats: &[],
    },
};
