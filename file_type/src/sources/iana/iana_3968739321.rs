use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3968739321: FileFormat = FileFormat {
    id: 3_968_739_321,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-mbms-usage-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-mbms-usage-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
