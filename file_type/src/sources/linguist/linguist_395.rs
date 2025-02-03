use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_395: FileFormat = FileFormat {
    id: 395,
    source_type: SourceType::Linguist,
    name: "WebIDL",
    extensions: &["webidl"],
    media_types: &["text/x-webidl"],
    internal_signatures: &[],
    related_formats: &[],
};
