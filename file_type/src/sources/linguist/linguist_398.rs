use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_398: FileFormat = FileFormat {
    id: 398,
    source_type: SourceType::Linguist,
    name: "XC",
    extensions: &["xc"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
