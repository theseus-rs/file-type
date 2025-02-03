use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_317: FileFormat = FileFormat {
    id: 317,
    source_type: SourceType::Linguist,
    name: "Ragel",
    extensions: &["rl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
