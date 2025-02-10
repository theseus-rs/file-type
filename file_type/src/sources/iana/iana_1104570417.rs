use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1104570417: FileType = FileType {
    file_format: &FileFormat {
        id: 1_104_570_417,
        source_type: SourceType::Iana,
        name: "disposition-notification",
        extensions: &[],
        media_types: &["message/disposition-notification"],
        signatures: &[],
        related_formats: &[],
    },
};
