use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_521429430: FileType = FileType {
    file_format: &FileFormat {
        id: 521_429_430,
        source_type: SourceType::Linguist,
        name: "Nearley",
        extensions: &["ne", "nearley"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
