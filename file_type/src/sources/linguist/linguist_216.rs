use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_216: FileFormat = FileFormat {
    id: 216,
    source_type: SourceType::Linguist,
    name: "M4Sugar",
    extensions: &["m4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
