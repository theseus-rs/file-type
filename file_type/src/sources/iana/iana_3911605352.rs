use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3911605352: FileType = FileType {
    file_format: &FileFormat {
        id: 3_911_605_352,
        source_type: SourceType::Iana,
        name: "DCD",
        extensions: &[],
        media_types: &["application/DCD"],
        signatures: &[],
        related_formats: &[],
    },
};
