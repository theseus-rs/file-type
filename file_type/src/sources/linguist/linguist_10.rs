use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_10: FileFormat = FileFormat {
    id: 10,
    source_type: SourceType::Linguist,
    name: "ActionScript",
    extensions: &["as"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
