use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_398752703: FileType = FileType {
    file_format: &FileFormat {
        id: 398_752_703,
        source_type: SourceType::Iana,
        name: "vnd.sycle+xml",
        extensions: &[],
        media_types: &["application/vnd.sycle+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
