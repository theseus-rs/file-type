use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_24: FileFormat = FileFormat {
    id: 24,
    source_type: SourceType::Linguist,
    name: "Assembly",
    extensions: &["a51", "asm", "i", "inc", "nas", "nasm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
