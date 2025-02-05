use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_340: FileFormat = FileFormat {
    id: 340,
    source_type: SourceType::Linguist,
    name: "Sass",
    extensions: &["sass"],
    media_types: &["text/x-sass"],
    signatures: &[],
    related_formats: &[],
};
