use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_481192983: FileFormat = FileFormat {
    id: 481_192_983,
    source_type: SourceType::Linguist,
    name: "NEON",
    extensions: &["neon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
