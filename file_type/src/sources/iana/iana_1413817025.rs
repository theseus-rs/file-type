use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1413817025: FileType = FileType {
    file_format: &FileFormat {
        id: 1_413_817_025,
        source_type: SourceType::Iana,
        name: "vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
        extensions: &[],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
