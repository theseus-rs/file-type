use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_24470517: FileFormat = FileFormat {
    id: 24_470_517,
    source_type: SourceType::Linguist,
    name: "Survex data",
    extensions: &["svx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
