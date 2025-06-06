use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_10: FileType = FileType {
    file_format: &FileFormat {
        id: 10,
        source_type: SourceType::Linguist,
        name: "ActionScript",
        extensions: &["as"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
