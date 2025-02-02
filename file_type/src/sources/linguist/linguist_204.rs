use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_204: FileFormat = FileFormat {
    id: 204,
    source_type: SourceType::Linguist,
    name: "Liquid",
    extensions: &["liquid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
