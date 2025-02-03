use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_379: FileFormat = FileFormat {
    id: 379,
    source_type: SourceType::Linguist,
    name: "Unified Parallel C",
    extensions: &["upc"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
