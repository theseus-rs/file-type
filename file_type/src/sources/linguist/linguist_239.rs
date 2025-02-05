use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_239: FileFormat = FileFormat {
    id: 239,
    source_type: SourceType::Linguist,
    name: "Myghty",
    extensions: &["myt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
