use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_691605112: FileFormat = FileFormat {
    id: 691_605_112,
    source_type: SourceType::Linguist,
    name: "dircolors",
    extensions: &["dircolors"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
