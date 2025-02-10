use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_320: FileType = FileType {
    file_format: &FileFormat {
        id: 320,
        source_type: SourceType::Linguist,
        name: "Red",
        extensions: &["red", "reds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
