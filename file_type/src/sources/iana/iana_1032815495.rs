use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1032815495: FileType = FileType {
    file_format: &FileFormat {
        id: 1_032_815_495,
        source_type: SourceType::Iana,
        name: "atsc-dynamic-event-message",
        extensions: &[],
        media_types: &["application/atsc-dynamic-event-message"],
        signatures: &[],
        related_formats: &[],
    },
};
