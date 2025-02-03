use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_105187618: FileFormat = FileFormat {
    id: 105_187_618,
    source_type: SourceType::Linguist,
    name: "OASv2-yaml",
    extensions: &["yaml", "yml"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
