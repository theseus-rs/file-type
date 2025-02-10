use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2667311118: FileType = FileType {
    file_format: &FileFormat {
        id: 2_667_311_118,
        source_type: SourceType::Iana,
        name: "EVRC",
        extensions: &[],
        media_types: &["audio/EVRC"],
        signatures: &[],
        related_formats: &[],
    },
};
