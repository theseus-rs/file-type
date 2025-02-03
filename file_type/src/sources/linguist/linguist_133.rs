use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_133: FileFormat = FileFormat {
    id: 133,
    source_type: SourceType::Linguist,
    name: "Golo",
    extensions: &["golo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
