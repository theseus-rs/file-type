use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_238: FileFormat = FileFormat {
    id: 238,
    source_type: SourceType::Linguist,
    name: "MoonScript",
    extensions: &["moon"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
