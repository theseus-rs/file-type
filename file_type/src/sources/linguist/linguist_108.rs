use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_108: FileFormat = FileFormat {
    id: 108,
    source_type: SourceType::Linguist,
    name: "Factor",
    extensions: &["factor"],
    media_types: &["text/x-factor"],
    internal_signatures: &[],
    related_formats: &[],
};
