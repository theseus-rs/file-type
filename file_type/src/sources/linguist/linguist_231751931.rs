use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_231751931: FileFormat = FileFormat {
    id: 231_751_931,
    source_type: SourceType::Linguist,
    name: "Monkey C",
    extensions: &["mc"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
