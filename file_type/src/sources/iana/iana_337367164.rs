use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_337367164: FileFormat = FileFormat {
    id: 337_367_164,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-info+xml"],
    signatures: &[],
    related_formats: &[],
};
