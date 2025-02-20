use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3814459377: FileType = FileType {
    file_format: &FileFormat {
        id: 3_814_459_377,
        source_type: SourceType::Iana,
        name: "alternative",
        extensions: &[],
        media_types: &["multipart/alternative"],
        signatures: &[],
        related_formats: &[],
    },
};
