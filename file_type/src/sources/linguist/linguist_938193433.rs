use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_938193433: FileFormat = FileFormat {
    id: 938_193_433,
    source_type: SourceType::Linguist,
    name: "MiniZinc Data",
    extensions: &["dzn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
