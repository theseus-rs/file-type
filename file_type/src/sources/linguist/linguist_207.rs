use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_207: FileFormat = FileFormat {
    id: 207,
    source_type: SourceType::Linguist,
    name: "Literate Haskell",
    extensions: &["lhs"],
    media_types: &["text/x-literate-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
