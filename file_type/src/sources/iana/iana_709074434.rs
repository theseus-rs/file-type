use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_709074434: FileType = FileType {
    file_format: &FileFormat {
        id: 709_074_434,
        source_type: SourceType::Iana,
        name: "heic",
        extensions: &[],
        media_types: &["image/heic"],
        signatures: &[],
        related_formats: &[],
    },
};
