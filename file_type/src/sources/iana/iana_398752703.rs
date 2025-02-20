use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
