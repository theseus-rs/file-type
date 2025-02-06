use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_118: FileFormat = FileFormat {
    id: 118,
    source_type: SourceType::Linguist,
    name: "GAMS",
    extensions: &["gms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
