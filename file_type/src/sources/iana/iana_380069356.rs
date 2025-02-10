use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_380069356: FileType = FileType {
    file_format: &FileFormat {
        id: 380_069_356,
        source_type: SourceType::Iana,
        name: "aces",
        extensions: &[],
        media_types: &["image/aces"],
        signatures: &[],
        related_formats: &[],
    },
};
