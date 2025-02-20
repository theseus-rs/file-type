use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_287: FileType = FileType {
    file_format: &FileFormat {
        id: 287,
        source_type: SourceType::Linguist,
        name: "Pike",
        extensions: &["pike", "pmod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
