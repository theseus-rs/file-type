use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_26941495: FileType = FileType {
    file_format: &FileFormat {
        id: 26_941_495,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-user-profile+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-user-profile+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
