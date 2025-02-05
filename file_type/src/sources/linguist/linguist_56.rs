use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_56: FileFormat = FileFormat {
    id: 56,
    source_type: SourceType::Linguist,
    name: "Charity",
    extensions: &["ch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
