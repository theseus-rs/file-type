use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_681406187: FileType = FileType {
    file_format: &FileFormat {
        id: 681_406_187,
        source_type: SourceType::Iana,
        name: "voice-message",
        extensions: &[],
        media_types: &["multipart/voice-message"],
        signatures: &[],
        related_formats: &[],
    },
};
