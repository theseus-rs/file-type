use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_604390837: FileFormat = FileFormat {
    id: 604_390_837,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-ue-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-ue-config+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
