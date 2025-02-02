use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_172: FileFormat = FileFormat {
    id: 172,
    source_type: SourceType::Linguist,
    name: "J",
    extensions: &["ijs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
