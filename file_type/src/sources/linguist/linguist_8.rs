use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_8: FileFormat = FileFormat {
    id: 8,
    source_type: SourceType::Linguist,
    name: "Classic ASP",
    extensions: &["asp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
