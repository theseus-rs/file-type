use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2877055551: FileType = FileType {
    file_format: &FileFormat {
        id: 2_877_055_551,
        source_type: SourceType::Iana,
        name: "vnd.as207960.vas.tap+uper",
        extensions: &[],
        media_types: &["application/vnd.as207960.vas.tap+uper"],
        signatures: &[],
        related_formats: &[],
    },
};
