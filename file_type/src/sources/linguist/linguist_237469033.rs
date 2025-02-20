use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_237469033: FileType = FileType {
    file_format: &FileFormat {
        id: 237_469_033,
        source_type: SourceType::Linguist,
        name: "Yul",
        extensions: &["yul"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
