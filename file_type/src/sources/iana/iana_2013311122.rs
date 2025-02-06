use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2013311122: FileFormat = FileFormat {
    id: 2_013_311_122,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-ue-init-config+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-ue-init-config+xml"],
    signatures: &[],
    related_formats: &[],
};
