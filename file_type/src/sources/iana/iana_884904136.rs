use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_884904136: FileType = FileType {
    file_format: &FileFormat {
        id: 884_904_136,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.mcptt-signed+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.mcptt-signed+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
