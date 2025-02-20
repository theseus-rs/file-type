use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_125: FileType = FileType {
    file_format: &FileFormat {
        id: 125,
        source_type: SourceType::Linguist,
        name: "Game Maker Language",
        extensions: &["gml"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
