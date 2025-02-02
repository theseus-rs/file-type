use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_218: FileFormat = FileFormat {
    id: 218,
    source_type: SourceType::Linguist,
    name: "MTML",
    extensions: &["mtml"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};
