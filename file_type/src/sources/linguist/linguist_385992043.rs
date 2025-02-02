use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_385992043: FileFormat = FileFormat {
    id: 385_992_043,
    source_type: SourceType::Linguist,
    name: "Mermaid",
    extensions: &["mermaid", "mmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
