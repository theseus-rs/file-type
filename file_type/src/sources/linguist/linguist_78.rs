use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_78: FileFormat = FileFormat {
    id: 78,
    source_type: SourceType::Linguist,
    name: "Cycript",
    extensions: &["cy"],
    media_types: &["text/javascript"],
    signatures: &[],
    related_formats: &[],
};
