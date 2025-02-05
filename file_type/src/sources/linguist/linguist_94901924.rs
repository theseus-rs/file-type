use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_94901924: FileFormat = FileFormat {
    id: 94_901_924,
    source_type: SourceType::Linguist,
    name: "TSX",
    extensions: &["tsx"],
    media_types: &["text/jsx"],
    signatures: &[],
    related_formats: &[],
};
