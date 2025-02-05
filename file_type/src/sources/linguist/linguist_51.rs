use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_51: FileFormat = FileFormat {
    id: 51,
    source_type: SourceType::Linguist,
    name: "CSV",
    extensions: &["csv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
