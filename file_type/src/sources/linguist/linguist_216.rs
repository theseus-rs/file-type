use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_216: FileType = FileType {
    file_format: &FileFormat {
        id: 216,
        source_type: SourceType::Linguist,
        name: "M4Sugar",
        extensions: &["m4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
