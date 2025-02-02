use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_25: FileFormat = FileFormat {
    id: 25,
    source_type: SourceType::Linguist,
    name: "Augeas",
    extensions: &["aug"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
