use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_356063509: FileFormat = FileFormat {
    id: 356_063_509,
    source_type: SourceType::Linguist,
    name: "CUE",
    extensions: &["cue"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
