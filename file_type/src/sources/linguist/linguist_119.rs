use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_119: FileFormat = FileFormat {
    id: 119,
    source_type: SourceType::Linguist,
    name: "GAP",
    extensions: &["g", "gap", "gd", "gi", "tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
