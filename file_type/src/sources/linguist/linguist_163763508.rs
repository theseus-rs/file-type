use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_163763508: FileType = FileType {
    file_format: &FileFormat {
        id: 163_763_508,
        source_type: SourceType::Linguist,
        name: "Clue",
        extensions: &["clue"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
