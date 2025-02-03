use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_114: FileFormat = FileFormat {
    id: 114,
    source_type: SourceType::Linguist,
    name: "Forth",
    extensions: &["4th", "f", "for", "forth", "fr", "frt", "fs", "fth"],
    media_types: &["text/x-forth"],
    internal_signatures: &[],
    related_formats: &[],
};
