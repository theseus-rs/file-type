use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_267963725: FileType = FileType {
    file_format: &FileFormat {
        id: 267_963_725,
        source_type: SourceType::Iana,
        name: "auth-policy+xml",
        extensions: &[],
        media_types: &["application/auth-policy+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
