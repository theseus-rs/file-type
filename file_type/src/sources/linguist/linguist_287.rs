use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_287: FileFormat = FileFormat {
    id: 287,
    source_type: SourceType::Linguist,
    name: "Pike",
    extensions: &["pike", "pmod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
