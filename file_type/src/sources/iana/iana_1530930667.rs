use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1530930667: FileType = FileType {
    file_format: &FileFormat {
        id: 1_530_930_667,
        source_type: SourceType::Iana,
        name: "mbms-schedule+xml",
        extensions: &[],
        media_types: &["application/mbms-schedule+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
