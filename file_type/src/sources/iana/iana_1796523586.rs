use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1796523586: FileFormat = FileFormat {
    id: 1_796_523_586,
    source_type: SourceType::Iana,
    name: "alto-endpointprop+json",
    extensions: &[],
    media_types: &["application/alto-endpointprop+json"],
    signatures: &[],
    related_formats: &[],
};
