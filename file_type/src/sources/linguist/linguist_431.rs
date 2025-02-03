use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_431: FileFormat = FileFormat {
    id: 431,
    source_type: SourceType::Linguist,
    name: "Ring",
    extensions: &["ring"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
