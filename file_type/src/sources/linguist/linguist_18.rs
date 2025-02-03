use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_18: FileFormat = FileFormat {
    id: 18,
    source_type: SourceType::Linguist,
    name: "Apollo Guidance Computer",
    extensions: &["agc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
