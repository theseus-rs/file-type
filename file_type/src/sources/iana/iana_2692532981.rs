use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2692532981: FileFormat = FileFormat {
    id: 2_692_532_981,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-location-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-location-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
