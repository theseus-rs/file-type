use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_452025714: FileType = FileType {
    file_format: &FileFormat {
        id: 452_025_714,
        source_type: SourceType::Linguist,
        name: "Elvish Transcript",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
