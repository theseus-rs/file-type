use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_267: FileFormat = FileFormat {
    id: 267,
    source_type: SourceType::Linguist,
    name: "Org",
    extensions: &["org"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
