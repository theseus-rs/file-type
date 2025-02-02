use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_686821385: FileFormat = FileFormat {
    id: 686_821_385,
    source_type: SourceType::Linguist,
    name: "Witcher Script",
    extensions: &["ws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
