use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_80: FileType = FileType {
    file_format: &FileFormat {
        id: 80,
        source_type: SourceType::Linguist,
        name: "D",
        extensions: &["d", "di"],
        media_types: &["text/x-d"],
        signatures: &[],
        related_formats: &[],
    },
};
