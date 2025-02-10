use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_356063509: FileType = FileType {
    file_format: &FileFormat {
        id: 356_063_509,
        source_type: SourceType::Linguist,
        name: "CUE",
        extensions: &["cue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
