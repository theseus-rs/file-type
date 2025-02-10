use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_132: FileType = FileType {
    file_format: &FileFormat {
        id: 132,
        source_type: SourceType::Linguist,
        name: "Go",
        extensions: &["go"],
        media_types: &["text/x-go"],
        signatures: &[],
        related_formats: &[],
    },
};
