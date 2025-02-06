use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_308: FileFormat = FileFormat {
    id: 308,
    source_type: SourceType::Linguist,
    name: "RAML",
    extensions: &["raml"],
    media_types: &["text/x-yaml"],
    signatures: &[],
    related_formats: &[],
};
