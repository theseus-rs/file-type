use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_80: FileFormat = FileFormat {
    id: 80,
    source_type: SourceType::Linguist,
    name: "D",
    extensions: &["d", "di"],
    media_types: &["text/x-d"],
    internal_signatures: &[],
    related_formats: &[],
};
