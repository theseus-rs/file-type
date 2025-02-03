use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2400235716: FileFormat = FileFormat {
    id: 2_400_235_716,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-service-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-service-config+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
