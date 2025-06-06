use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3923222572: FileType = FileType {
    file_format: &FileFormat {
        id: 3_923_222_572,
        source_type: SourceType::Iana,
        name: "vnd.groove-identity-message",
        extensions: &[],
        media_types: &["application/vnd.groove-identity-message"],
        signatures: &[],
        related_formats: &[],
    },
};
