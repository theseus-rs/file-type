use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_137: FileFormat = FileFormat {
    id: 137,
    source_type: SourceType::Linguist,
    name: "Grammatical Framework",
    extensions: &["gf"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
