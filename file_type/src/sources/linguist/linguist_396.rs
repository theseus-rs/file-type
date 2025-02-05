use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_396: FileFormat = FileFormat {
    id: 396,
    source_type: SourceType::Linguist,
    name: "World of Warcraft Addon Data",
    extensions: &["toc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
