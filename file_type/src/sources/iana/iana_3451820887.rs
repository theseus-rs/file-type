use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3451820887: FileFormat = FileFormat {
    id: 3_451_820_887,
    source_type: SourceType::Iana,
    name: "p21",
    extensions: &[],
    media_types: &["application/p21"],
    signatures: &[],
    related_formats: &[],
};
