use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_429: FileFormat = FileFormat {
    id: 429,
    source_type: SourceType::Linguist,
    name: "ABNF",
    extensions: &["abnf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
