use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_64: FileFormat = FileFormat {
    id: 64,
    source_type: SourceType::Linguist,
    name: "ColdFusion",
    extensions: &["cfm", "cfml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
