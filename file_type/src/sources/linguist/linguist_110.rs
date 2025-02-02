use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_110: FileFormat = FileFormat {
    id: 110,
    source_type: SourceType::Linguist,
    name: "Fantom",
    extensions: &["fan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
