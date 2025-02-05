use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_132: FileFormat = FileFormat {
    id: 132,
    source_type: SourceType::Linguist,
    name: "Go",
    extensions: &["go"],
    media_types: &["text/x-go"],
    signatures: &[],
    related_formats: &[],
};
