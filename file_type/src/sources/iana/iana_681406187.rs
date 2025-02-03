use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_681406187: FileFormat = FileFormat {
    id: 681_406_187,
    source_type: SourceType::Iana,
    name: "voice-message",
    extensions: &[],
    media_types: &["multipart/voice-message"],
    internal_signatures: &[],
    related_formats: &[],
};
