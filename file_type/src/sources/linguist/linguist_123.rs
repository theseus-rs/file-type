use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_123: FileFormat = FileFormat {
    id: 123,
    source_type: SourceType::Linguist,
    name: "GDScript",
    extensions: &["gd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
