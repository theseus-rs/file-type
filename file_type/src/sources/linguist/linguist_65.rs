use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_65: FileFormat = FileFormat {
    id: 65,
    source_type: SourceType::Linguist,
    name: "ColdFusion CFC",
    extensions: &["cfc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
