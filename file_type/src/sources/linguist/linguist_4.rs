use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_4: FileType = FileType {
    file_format: &FileFormat {
        id: 4,
        source_type: SourceType::Linguist,
        name: "ANTLR",
        extensions: &["g4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
