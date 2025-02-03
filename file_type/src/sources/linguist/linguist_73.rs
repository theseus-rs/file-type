use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_73: FileFormat = FileFormat {
    id: 73,
    source_type: SourceType::Linguist,
    name: "Csound",
    extensions: &["orc", "udo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
