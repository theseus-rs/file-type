use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_431: FileType = FileType {
    file_format: &FileFormat {
        id: 431,
        source_type: SourceType::Linguist,
        name: "Ring",
        extensions: &["ring"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
