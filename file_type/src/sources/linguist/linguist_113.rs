use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_113: FileFormat = FileFormat {
    id: 113,
    source_type: SourceType::Linguist,
    name: "Formatted",
    extensions: &["eam.fs", "for"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
