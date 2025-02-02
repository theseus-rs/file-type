use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_361: FileFormat = FileFormat {
    id: 361,
    source_type: SourceType::Linguist,
    name: "SuperCollider",
    extensions: &["sc", "scd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
