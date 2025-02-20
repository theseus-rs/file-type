use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
