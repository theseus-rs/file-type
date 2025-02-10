use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_638334599: FileType = FileType {
    file_format: &FileFormat {
        id: 638_334_599,
        source_type: SourceType::Linguist,
        name: "Move",
        extensions: &["move"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
