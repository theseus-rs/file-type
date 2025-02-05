use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_321: FileFormat = FileFormat {
    id: 321,
    source_type: SourceType::Linguist,
    name: "Redcode",
    extensions: &["cw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
