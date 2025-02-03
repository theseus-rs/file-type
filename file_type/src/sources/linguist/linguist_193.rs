use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_193: FileFormat = FileFormat {
    id: 193,
    source_type: SourceType::Linguist,
    name: "LSL",
    extensions: &["lsl", "lslp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
