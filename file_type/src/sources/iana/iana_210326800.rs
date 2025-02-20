use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_210326800: FileType = FileType {
    file_format: &FileFormat {
        id: 210_326_800,
        source_type: SourceType::Iana,
        name: "VP8",
        extensions: &[],
        media_types: &["video/VP8"],
        signatures: &[],
        related_formats: &[],
    },
};
