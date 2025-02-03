use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_370: FileFormat = FileFormat {
    id: 370,
    source_type: SourceType::Linguist,
    name: "Tea",
    extensions: &["tea"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
