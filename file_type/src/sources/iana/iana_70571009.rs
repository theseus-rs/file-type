use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_70571009: FileFormat = FileFormat {
    id: 70_571_009,
    source_type: SourceType::Iana,
    name: "dssc+der",
    extensions: &[],
    media_types: &["application/dssc+der"],
    signatures: &[],
    related_formats: &[],
};
