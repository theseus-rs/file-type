use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_406: FileFormat = FileFormat {
    id: 406,
    source_type: SourceType::Linguist,
    name: "Xtend",
    extensions: &["xtend"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
