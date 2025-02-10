use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_61: FileType = FileType {
    file_format: &FileFormat {
        id: 61,
        source_type: SourceType::Linguist,
        name: "Click",
        extensions: &["click"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
