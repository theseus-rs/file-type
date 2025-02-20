use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_638640543: FileType = FileType {
    file_format: &FileFormat {
        id: 638_640_543,
        source_type: SourceType::Iana,
        name: "csta+xml",
        extensions: &[],
        media_types: &["application/csta+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
