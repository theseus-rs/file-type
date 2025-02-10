use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2693170543: FileType = FileType {
    file_format: &FileFormat {
        id: 2_693_170_543,
        source_type: SourceType::Iana,
        name: "sfnt",
        extensions: &[],
        media_types: &["font/sfnt"],
        signatures: &[],
        related_formats: &[],
    },
};
