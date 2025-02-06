use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_166: FileFormat = FileFormat {
    id: 166,
    source_type: SourceType::Linguist,
    name: "Inform 7",
    extensions: &["i7x", "ni"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
