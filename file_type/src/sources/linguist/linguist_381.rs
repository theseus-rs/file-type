use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_381: FileFormat = FileFormat {
    id: 381,
    source_type: SourceType::Linguist,
    name: "Uno",
    extensions: &["uno"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};
