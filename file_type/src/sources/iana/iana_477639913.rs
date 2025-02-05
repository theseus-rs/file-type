use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_477639913: FileFormat = FileFormat {
    id: 477_639_913,
    source_type: SourceType::Iana,
    name: "alto-tips+json",
    extensions: &[],
    media_types: &["application/alto-tips+json"],
    signatures: &[],
    related_formats: &[],
};
