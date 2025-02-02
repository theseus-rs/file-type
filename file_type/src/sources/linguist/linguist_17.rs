use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_17: FileFormat = FileFormat {
    id: 17,
    source_type: SourceType::Linguist,
    name: "Apex",
    extensions: &["apex", "cls", "trigger"],
    media_types: &["text/x-java"],
    internal_signatures: &[],
    related_formats: &[],
};
