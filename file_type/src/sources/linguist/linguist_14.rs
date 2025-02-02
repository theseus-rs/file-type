use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_14: FileFormat = FileFormat {
    id: 14,
    source_type: SourceType::Linguist,
    name: "Alpine Abuild",
    extensions: &[],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
