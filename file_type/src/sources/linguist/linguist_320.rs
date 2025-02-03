use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_320: FileFormat = FileFormat {
    id: 320,
    source_type: SourceType::Linguist,
    name: "Red",
    extensions: &["red", "reds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
