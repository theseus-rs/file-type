use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_375: FileFormat = FileFormat {
    id: 375,
    source_type: SourceType::Linguist,
    name: "Turing",
    extensions: &["t", "tu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
