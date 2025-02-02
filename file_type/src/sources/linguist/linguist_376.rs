use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_376: FileFormat = FileFormat {
    id: 376,
    source_type: SourceType::Linguist,
    name: "Turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    internal_signatures: &[],
    related_formats: &[],
};
