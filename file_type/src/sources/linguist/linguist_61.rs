use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_61: FileFormat = FileFormat {
    id: 61,
    source_type: SourceType::Linguist,
    name: "Click",
    extensions: &["click"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
