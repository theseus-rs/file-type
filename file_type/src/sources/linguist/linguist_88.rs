use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_88: FileFormat = FileFormat {
    id: 88,
    source_type: SourceType::Linguist,
    name: "Diff",
    extensions: &["diff", "patch"],
    media_types: &["text/x-diff"],
    internal_signatures: &[],
    related_formats: &[],
};
