use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_417: FileFormat = FileFormat {
    id: 417,
    source_type: SourceType::Linguist,
    name: "nesC",
    extensions: &["nc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
