use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_177: FileFormat = FileFormat {
    id: 177,
    source_type: SourceType::Linguist,
    name: "JSONiq",
    extensions: &["jq"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
