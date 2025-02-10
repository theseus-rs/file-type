use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_948240628: FileType = FileType {
    file_format: &FileFormat {
        id: 948_240_628,
        source_type: SourceType::Iana,
        name: "mpeg",
        extensions: &[],
        media_types: &["video/mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
