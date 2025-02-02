use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_318: FileFormat = FileFormat {
    id: 318,
    source_type: SourceType::Linguist,
    name: "Raw token data",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
