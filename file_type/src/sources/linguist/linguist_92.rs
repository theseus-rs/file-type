use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_92: FileFormat = FileFormat {
    id: 92,
    source_type: SourceType::Linguist,
    name: "E",
    extensions: &["e"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
