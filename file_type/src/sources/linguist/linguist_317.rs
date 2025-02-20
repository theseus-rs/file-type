use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_317: FileType = FileType {
    file_format: &FileFormat {
        id: 317,
        source_type: SourceType::Linguist,
        name: "Ragel",
        extensions: &["rl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
