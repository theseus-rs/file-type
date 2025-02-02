use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_243: FileFormat = FileFormat {
    id: 243,
    source_type: SourceType::Linguist,
    name: "Nemerle",
    extensions: &["n"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
