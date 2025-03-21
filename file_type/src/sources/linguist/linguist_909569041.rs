use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_909569041: FileType = FileType {
    file_format: &FileFormat {
        id: 909_569_041,
        source_type: SourceType::Linguist,
        name: "BibTeX Style",
        extensions: &["bst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
