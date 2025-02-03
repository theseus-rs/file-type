use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_74: FileFormat = FileFormat {
    id: 74,
    source_type: SourceType::Linguist,
    name: "Csound Document",
    extensions: &["csd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
