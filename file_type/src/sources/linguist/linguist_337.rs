use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_337: FileFormat = FileFormat {
    id: 337,
    source_type: SourceType::Linguist,
    name: "SVG",
    extensions: &["svg"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
