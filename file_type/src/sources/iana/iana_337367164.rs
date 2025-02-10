use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_337367164: FileType = FileType {
    file_format: &FileFormat {
        id: 337_367_164,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
