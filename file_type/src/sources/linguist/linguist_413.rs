use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_413: FileType = FileType {
    file_format: &FileFormat {
        id: 413,
        source_type: SourceType::Linguist,
        name: "eC",
        extensions: &["ec", "eh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
