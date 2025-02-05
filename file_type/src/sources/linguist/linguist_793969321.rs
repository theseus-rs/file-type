use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_793969321: FileFormat = FileFormat {
    id: 793_969_321,
    source_type: SourceType::Linguist,
    name: "Dhall",
    extensions: &["dhall"],
    media_types: &["text/x-haskell"],
    signatures: &[],
    related_formats: &[],
};
