use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3010902088: FileType = FileType {
    file_format: &FileFormat {
        id: 3_010_902_088,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-regroup+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-regroup+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
