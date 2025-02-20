use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
