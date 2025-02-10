use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2389939401: FileType = FileType {
    file_format: &FileFormat {
        id: 2_389_939_401,
        source_type: SourceType::Iana,
        name: "vnd.ms-powerpoint.presentation.macroEnabled.12",
        extensions: &[],
        media_types: &["application/vnd.ms-powerpoint.presentation.macroEnabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
