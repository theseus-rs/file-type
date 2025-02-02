use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_87: FileFormat = FileFormat {
    id: 87,
    source_type: SourceType::Linguist,
    name: "Dart",
    extensions: &["dart"],
    media_types: &["application/dart"],
    internal_signatures: &[],
    related_formats: &[],
};
