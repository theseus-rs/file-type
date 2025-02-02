use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_415: FileFormat = FileFormat {
    id: 415,
    source_type: SourceType::Linguist,
    name: "fish",
    extensions: &["fish"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
