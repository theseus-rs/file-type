use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4083711601: FileFormat = FileFormat {
    id: 4_083_711_601,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-user-profile+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-user-profile+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
