use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_29465275: FileType = FileType {
    file_format: &FileFormat {
        id: 29_465_275,
        source_type: SourceType::Iana,
        name: "vnd.trueapp",
        extensions: &[],
        media_types: &["application/vnd.trueapp"],
        signatures: &[],
        related_formats: &[],
    },
};
