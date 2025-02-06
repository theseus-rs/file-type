use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_59: FileFormat = FileFormat {
    id: 59,
    source_type: SourceType::Linguist,
    name: "Clarion",
    extensions: &["clw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
