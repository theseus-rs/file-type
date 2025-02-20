use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_83: FileType = FileType {
    file_format: &FileFormat {
        id: 83,
        source_type: SourceType::Linguist,
        name: "DM",
        extensions: &["dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
