use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_418: FileFormat = FileFormat {
    id: 418,
    source_type: SourceType::Linguist,
    name: "ooc",
    extensions: &["ooc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
