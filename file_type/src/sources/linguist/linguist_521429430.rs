use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_521429430: FileFormat = FileFormat {
    id: 521_429_430,
    source_type: SourceType::Linguist,
    name: "Nearley",
    extensions: &["ne", "nearley"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
