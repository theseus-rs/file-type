use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_96: FileFormat = FileFormat {
    id: 96,
    source_type: SourceType::Linguist,
    name: "EQ",
    extensions: &["eq"],
    media_types: &["text/x-csharp"],
    signatures: &[],
    related_formats: &[],
};
