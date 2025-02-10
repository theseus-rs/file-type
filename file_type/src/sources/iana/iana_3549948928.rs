use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3549948928: FileType = FileType {
    file_format: &FileFormat {
        id: 3_549_948_928,
        source_type: SourceType::Iana,
        name: "pls+xml",
        extensions: &[],
        media_types: &["application/pls+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
