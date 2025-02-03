use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_366: FileFormat = FileFormat {
    id: 366,
    source_type: SourceType::Linguist,
    name: "TXL",
    extensions: &["txl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
