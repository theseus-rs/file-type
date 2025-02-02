use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_162: FileFormat = FileFormat {
    id: 162,
    source_type: SourceType::Linguist,
    name: "IGOR Pro",
    extensions: &["ipf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
