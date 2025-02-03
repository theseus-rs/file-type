use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_192: FileFormat = FileFormat {
    id: 192,
    source_type: SourceType::Linguist,
    name: "LOLCODE",
    extensions: &["lol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
