use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_373: FileFormat = FileFormat {
    id: 373,
    source_type: SourceType::Linguist,
    name: "Textile",
    extensions: &["textile"],
    media_types: &["text/x-textile"],
    internal_signatures: &[],
    related_formats: &[],
};
