use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_391: FileFormat = FileFormat {
    id: 391,
    source_type: SourceType::Linguist,
    name: "Vue",
    extensions: &["vue"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
