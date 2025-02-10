use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_383: FileType = FileType {
    file_format: &FileFormat {
        id: 383,
        source_type: SourceType::Linguist,
        name: "UrWeb",
        extensions: &["ur", "urs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
