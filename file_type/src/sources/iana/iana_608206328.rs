use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_608206328: FileFormat = FileFormat {
    id: 608_206_328,
    source_type: SourceType::Iana,
    name: "alto-updatestreamcontrol+json",
    extensions: &[],
    media_types: &["application/alto-updatestreamcontrol+json"],
    internal_signatures: &[],
    related_formats: &[],
};
