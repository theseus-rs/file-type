use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_219: FileFormat = FileFormat {
    id: 219,
    source_type: SourceType::Linguist,
    name: "MUF",
    extensions: &["m", "muf"],
    media_types: &["text/x-forth"],
    internal_signatures: &[],
    related_formats: &[],
};
