use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_415: FileType = FileType {
    file_format: &FileFormat {
        id: 415,
        source_type: SourceType::Linguist,
        name: "fish",
        extensions: &["fish"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
