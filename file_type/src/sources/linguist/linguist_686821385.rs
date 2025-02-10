use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_686821385: FileType = FileType {
    file_format: &FileFormat {
        id: 686_821_385,
        source_type: SourceType::Linguist,
        name: "Witcher Script",
        extensions: &["ws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
