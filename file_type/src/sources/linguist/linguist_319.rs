use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_319: FileFormat = FileFormat {
    id: 319,
    source_type: SourceType::Linguist,
    name: "Rebol",
    extensions: &["r", "r2", "r3", "reb", "rebol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
