use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_620599567: FileFormat = FileFormat {
    id: 620_599_567,
    source_type: SourceType::Linguist,
    name: "Cairo",
    extensions: &["cairo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
