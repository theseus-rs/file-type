use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1182453069: FileType = FileType {
    file_format: &FileFormat {
        id: 1_182_453_069,
        source_type: SourceType::Iana,
        name: "vnd.3gpp.pinapp-info+xml",
        extensions: &[],
        media_types: &["application/vnd.3gpp.pinapp-info+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
