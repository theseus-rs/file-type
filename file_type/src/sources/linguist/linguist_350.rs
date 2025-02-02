use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_350: FileFormat = FileFormat {
    id: 350,
    source_type: SourceType::Linguist,
    name: "Slim",
    extensions: &["slim"],
    media_types: &["text/x-slim"],
    internal_signatures: &[],
    related_formats: &[],
};
