use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_188: FileFormat = FileFormat {
    id: 188,
    source_type: SourceType::Linguist,
    name: "Kit",
    extensions: &["kit"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};
