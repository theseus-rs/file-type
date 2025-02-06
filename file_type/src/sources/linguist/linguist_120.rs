use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_120: FileFormat = FileFormat {
    id: 120,
    source_type: SourceType::Linguist,
    name: "Unix Assembly",
    extensions: &["ms", "s"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
