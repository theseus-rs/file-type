use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_12: FileFormat = FileFormat {
    id: 12,
    source_type: SourceType::Linguist,
    name: "Agda",
    extensions: &["agda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
