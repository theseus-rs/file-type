use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_974514097: FileFormat = FileFormat {
    id: 974_514_097,
    source_type: SourceType::Linguist,
    name: "DataWeave",
    extensions: &["dwl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
