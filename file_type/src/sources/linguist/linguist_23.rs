use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_23: FileFormat = FileFormat {
    id: 23,
    source_type: SourceType::Linguist,
    name: "AspectJ",
    extensions: &["aj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
