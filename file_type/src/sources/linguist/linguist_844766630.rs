use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_844766630: FileFormat = FileFormat {
    id: 844_766_630,
    source_type: SourceType::Linguist,
    name: "Ecmarkup",
    extensions: &["html"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
