use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_60: FileFormat = FileFormat {
    id: 60,
    source_type: SourceType::Linguist,
    name: "Clean",
    extensions: &["dcl", "icl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
