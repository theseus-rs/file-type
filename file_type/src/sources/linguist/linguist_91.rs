use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_91: FileFormat = FileFormat {
    id: 91,
    source_type: SourceType::Linguist,
    name: "Dylan",
    extensions: &["dyl", "dylan", "intr", "lid"],
    media_types: &["text/x-dylan"],
    internal_signatures: &[],
    related_formats: &[],
};
