use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_237827459: FileType = FileType {
    file_format: &FileFormat {
        id: 237_827_459,
        source_type: SourceType::Iana,
        name: "imdn+xml",
        extensions: &[],
        media_types: &["message/imdn+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
