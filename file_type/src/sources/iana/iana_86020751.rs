use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_86020751: FileFormat = FileFormat {
    id: 86_020_751,
    source_type: SourceType::Iana,
    name: "PDX",
    extensions: &[],
    media_types: &["application/PDX"],
    signatures: &[],
    related_formats: &[],
};
