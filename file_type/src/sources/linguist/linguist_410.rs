use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_410: FileFormat = FileFormat {
    id: 410,
    source_type: SourceType::Linguist,
    name: "Zephir",
    extensions: &["zep"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
