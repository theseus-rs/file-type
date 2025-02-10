use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_230681993: FileType = FileType {
    file_format: &FileFormat {
        id: 230_681_993,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-location-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-location-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
