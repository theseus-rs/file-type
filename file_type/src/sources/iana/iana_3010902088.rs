use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3010902088: FileFormat = FileFormat {
    id: 3_010_902_088,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-regroup+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-regroup+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
