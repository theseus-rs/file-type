use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_178322513: FileType = FileType {
    file_format: &FileFormat {
        id: 178_322_513,
        source_type: SourceType::Linguist,
        name: "Nasal",
        extensions: &["nas"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
