use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_462488745: FileFormat = FileFormat {
    id: 462_488_745,
    source_type: SourceType::Linguist,
    name: "mcfunction",
    extensions: &["mcfunction"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
