use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_154: FileFormat = FileFormat {
    id: 154,
    source_type: SourceType::Linguist,
    name: "Haml",
    extensions: &["haml", "haml.deface"],
    media_types: &["text/x-haml"],
    signatures: &[],
    related_formats: &[],
};
