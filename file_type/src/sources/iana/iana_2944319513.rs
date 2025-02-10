use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2944319513: FileType = FileType {
    file_format: &FileFormat {
        id: 2_944_319_513,
        source_type: SourceType::Iana,
        name: "vnd.pwg-xhtml-print+xml",
        extensions: &[],
        media_types: &["application/vnd.pwg-xhtml-print+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
