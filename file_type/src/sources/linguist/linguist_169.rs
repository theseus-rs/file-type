use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_169: FileFormat = FileFormat {
    id: 169,
    source_type: SourceType::Linguist,
    name: "Ioke",
    extensions: &["ik"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
