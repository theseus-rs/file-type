use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_125: FileFormat = FileFormat {
    id: 125,
    source_type: SourceType::Linguist,
    name: "Game Maker Language",
    extensions: &["gml"],
    media_types: &["text/x-c++src"],
    signatures: &[],
    related_formats: &[],
};
