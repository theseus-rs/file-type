use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_230681993: FileFormat = FileFormat {
    id: 230_681_993,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-location-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-location-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
