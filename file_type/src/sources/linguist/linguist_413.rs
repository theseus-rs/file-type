use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_413: FileFormat = FileFormat {
    id: 413,
    source_type: SourceType::Linguist,
    name: "eC",
    extensions: &["ec", "eh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
