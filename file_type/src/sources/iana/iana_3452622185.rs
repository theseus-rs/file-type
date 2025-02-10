use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3452622185: FileType = FileType {
    file_format: &FileFormat {
        id: 3_452_622_185,
        source_type: SourceType::Iana,
        name: "example",
        extensions: &[],
        media_types: &["video/example"],
        signatures: &[],
        related_formats: &[],
    },
};
