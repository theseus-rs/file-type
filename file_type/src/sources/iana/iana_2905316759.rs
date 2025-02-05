use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2905316759: FileFormat = FileFormat {
    id: 2_905_316_759,
    source_type: SourceType::Iana,
    name: "vnd.hal+json",
    extensions: &[],
    media_types: &["application/vnd.hal+json"],
    signatures: &[],
    related_formats: &[],
};
