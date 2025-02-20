use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_361: FileType = FileType {
    file_format: &FileFormat {
        id: 361,
        source_type: SourceType::Linguist,
        name: "SuperCollider",
        extensions: &["sc", "scd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
