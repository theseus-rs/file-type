use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_157: FileFormat = FileFormat {
    id: 157,
    source_type: SourceType::Linguist,
    name: "Haskell",
    extensions: &["hs", "hs-boot", "hsc"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
