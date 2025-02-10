use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_319: FileType = FileType {
    file_format: &FileFormat {
        id: 319,
        source_type: SourceType::Linguist,
        name: "Rebol",
        extensions: &["r", "r2", "r3", "reb", "rebol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
