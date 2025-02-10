use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1797176887: FileType = FileType {
    file_format: &FileFormat {
        id: 1_797_176_887,
        source_type: SourceType::Iana,
        name: "jpm",
        extensions: &[],
        media_types: &["image/jpm"],
        signatures: &[],
        related_formats: &[],
    },
};
