use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_234: FileFormat = FileFormat {
    id: 234,
    source_type: SourceType::Linguist,
    name: "Modula-2",
    extensions: &["mod"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
