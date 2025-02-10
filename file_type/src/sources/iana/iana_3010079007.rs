use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3010079007: FileType = FileType {
    file_format: &FileFormat {
        id: 3_010_079_007,
        source_type: SourceType::Iana,
        name: "eat-ucs+json",
        extensions: &[],
        media_types: &["application/eat-ucs+json"],
        signatures: &[],
        related_formats: &[],
    },
};
