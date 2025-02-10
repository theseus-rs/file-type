use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2530088001: FileType = FileType {
    file_format: &FileFormat {
        id: 2_530_088_001,
        source_type: SourceType::Iana,
        name: "voicexml+xml",
        extensions: &[],
        media_types: &["application/voicexml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
