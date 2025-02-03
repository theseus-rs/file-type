use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_58: FileFormat = FileFormat {
    id: 58,
    source_type: SourceType::Linguist,
    name: "Cirru",
    extensions: &["cirru"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
