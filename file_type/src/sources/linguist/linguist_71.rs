use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_71: FileFormat = FileFormat {
    id: 71,
    source_type: SourceType::Linguist,
    name: "Creole",
    extensions: &["creole"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
