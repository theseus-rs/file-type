use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_45: FileFormat = FileFormat {
    id: 45,
    source_type: SourceType::Linguist,
    name: "C2hs Haskell",
    extensions: &["chs"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
