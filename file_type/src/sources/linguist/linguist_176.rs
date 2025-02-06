use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_176: FileFormat = FileFormat {
    id: 176,
    source_type: SourceType::Linguist,
    name: "JSONLD",
    extensions: &["jsonld"],
    media_types: &["application/json"],
    signatures: &[],
    related_formats: &[],
};
