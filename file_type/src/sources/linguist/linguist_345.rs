use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_345: FileFormat = FileFormat {
    id: 345,
    source_type: SourceType::Linguist,
    name: "Self",
    extensions: &["self"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
