use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_362: FileType = FileType {
    file_format: &FileFormat {
        id: 362,
        source_type: SourceType::Linguist,
        name: "Swift",
        extensions: &["swift"],
        media_types: &["text/x-swift"],
        signatures: &[],
        related_formats: &[],
    },
};
