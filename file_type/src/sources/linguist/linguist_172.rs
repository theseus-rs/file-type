use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_172: FileType = FileType {
    file_format: &FileFormat {
        id: 172,
        source_type: SourceType::Linguist,
        name: "J",
        extensions: &["ijs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
