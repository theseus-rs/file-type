use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_28: FileFormat = FileFormat {
    id: 28,
    source_type: SourceType::Linguist,
    name: "Awk",
    extensions: &["auk", "awk", "gawk", "mawk", "nawk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
