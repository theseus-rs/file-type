use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_4896465: FileFormat = FileFormat {
    id: 4_896_465,
    source_type: SourceType::Linguist,
    name: "MiniYAML",
    extensions: &["yaml", "yml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
