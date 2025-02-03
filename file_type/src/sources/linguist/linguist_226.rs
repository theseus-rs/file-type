use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_226: FileFormat = FileFormat {
    id: 226,
    source_type: SourceType::Linguist,
    name: "Maven POM",
    extensions: &[],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
