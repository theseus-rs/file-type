use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_236: FileFormat = FileFormat {
    id: 236,
    source_type: SourceType::Linguist,
    name: "Monkey",
    extensions: &["monkey", "monkey2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
