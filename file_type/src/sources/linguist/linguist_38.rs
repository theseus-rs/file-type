use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_38: FileFormat = FileFormat {
    id: 38,
    source_type: SourceType::Linguist,
    name: "Brainfuck",
    extensions: &["b", "bf"],
    media_types: &["text/x-brainfuck"],
    signatures: &[],
    related_formats: &[],
};
