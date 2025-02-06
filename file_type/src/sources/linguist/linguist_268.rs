use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_268: FileFormat = FileFormat {
    id: 268,
    source_type: SourceType::Linguist,
    name: "Ox",
    extensions: &["ox", "oxh", "oxo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
