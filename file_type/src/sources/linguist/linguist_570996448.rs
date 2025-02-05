use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_570996448: FileFormat = FileFormat {
    id: 570_996_448,
    source_type: SourceType::Linguist,
    name: "Elvish",
    extensions: &["elv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
