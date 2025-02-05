use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3681756992: FileFormat = FileFormat {
    id: 3_681_756_992,
    source_type: SourceType::Iana,
    name: "lostsync+xml",
    extensions: &[],
    media_types: &["application/lostsync+xml"],
    signatures: &[],
    related_formats: &[],
};
