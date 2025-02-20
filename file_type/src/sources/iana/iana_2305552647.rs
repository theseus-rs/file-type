use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2305552647: FileType = FileType {
    file_format: &FileFormat {
        id: 2_305_552_647,
        source_type: SourceType::Iana,
        name: "http",
        extensions: &[],
        media_types: &["message/http"],
        signatures: &[],
        related_formats: &[],
    },
};
