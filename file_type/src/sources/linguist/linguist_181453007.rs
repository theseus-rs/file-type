use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_181453007: FileFormat = FileFormat {
    id: 181_453_007,
    source_type: SourceType::Linguist,
    name: "MoonBit",
    extensions: &["mbt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
