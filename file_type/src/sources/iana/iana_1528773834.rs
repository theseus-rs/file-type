use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1528773834: FileFormat = FileFormat {
    id: 1_528_773_834,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcptt-affiliation-command+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcptt-affiliation-command+xml"],
    signatures: &[],
    related_formats: &[],
};
